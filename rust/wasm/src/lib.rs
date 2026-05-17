mod local_storage;
mod webusb;

use std::sync::{Mutex, OnceLock};

use idevice::{IdeviceService, lockdown::LockdownClient};
use iloader_core::{
    device::{ConnectionType, DeviceInfo},
    error::{AppError, WasmError},
    read_lockdown_values,
};
use isideload::{
    anisette::remote_v3::RemoteV3AnisetteProvider,
    auth::builder::AppleAccountBuilder,
    dev::developer_session::DeveloperSession,
    sideload::{SideloaderBuilder, sideloader::Sideloader},
};
use netmuxd::usb::provider::UsbMuxProvider;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::{local_storage::LocalStorage, webusb::get_webusb_provider};

static IDEVICE: OnceLock<Mutex<Option<UsbMuxProvider>>> = OnceLock::new();
static SIDELOADER: OnceLock<Mutex<Option<Sideloader>>> = OnceLock::new();

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    let _ = isideload::init();
}

#[wasm_bindgen]
pub async fn get_devices() -> Result<JsValue, WasmError> {
    let provider = get_webusb_provider("iloader-web").await?;

    let mut lockdown_client = LockdownClient::connect(&provider).await.map_err(|e| {
        console::error_1(&format!("Unable to connect to lockdown: {e:?}").into());
        AppError::DeviceComsWithMessage("Unable to connect to lockdown".into(), e.to_string())
    })?;

    let device_name_value = lockdown_client
        .get_value(Some("DeviceName"), None)
        .await
        .map_err(|e| {
            console::error_1(&format!("Failed to fetch DeviceName: {e:?}").into());
            AppError::DeviceComsWithMessage("Failed to fetch DeviceName".into(), e.to_string())
        })?;

    let device_name = device_name_value.as_string().ok_or_else(|| {
        console::error_1(&format!("DeviceName was not a string").into());
        AppError::DeviceComs("DeviceName was not a string".into())
    })?;

    let version_value = lockdown_client
        .get_value(Some("ProductVersion"), None)
        .await
        .map_err(|e| {
            console::error_1(&format!("Failed to fetch ProductVersion: {e:?}").into());
            AppError::DeviceComsWithMessage("Failed to fetch ProductVersion".into(), e.to_string())
        })?;

    let version = version_value.as_string().ok_or_else(|| {
        console::error_1(&format!("ProductVersion was not a string").into());
        AppError::DeviceComs("Product version was not a string".into())
    })?;

    let udid_value = lockdown_client
        .get_value(Some("UniqueDeviceID"), None)
        .await
        .map_err(|e| {
            console::error_1(&format!("Failed to fetch UniqueDeviceID: {e:?}").into());
            AppError::DeviceComsWithMessage("Failed to fetch UniqueDeviceID".into(), e.to_string())
        })?;

    let udid = udid_value.as_string().ok_or_else(|| {
        console::error_1(&format!("UniqueDeviceID was not a string").into());
        AppError::DeviceComs("UniqueDeviceID was not a string".into())
    })?;

    let devices: Vec<DeviceInfo> = vec![DeviceInfo {
        name: device_name.to_string(),
        udid: udid.to_string(),
        connection_type: ConnectionType::WebUSB,
        version: version.to_string(),
    }];

    let mutex = IDEVICE.get_or_init(|| Mutex::new(None));
    *mutex.lock().unwrap() = Some(provider);

    Ok(serde_wasm_bindgen::to_value(&devices)
        .map_err(|e| AppError::Misc(format!("serde to JsValue: {e:?}")))?)
}

#[wasm_bindgen]
pub async fn read_lockdown() -> Result<String, WasmError> {
    let mutex = IDEVICE.get_or_init(|| Mutex::new(None));
    let lock = mutex
        .lock()
        .map_err(|e| AppError::Misc(format!("Failed to lock IDEVICE mutex: {e:?}")))?;
    let provider = lock
        .as_ref()
        .ok_or_else(|| AppError::Misc("No device provider available".into()))?;

    let res = read_lockdown_values(provider).await?;
    Ok(res)
}

#[wasm_bindgen]
pub async fn login(
    email: String,
    password: String,
    two_factor_callback: js_sys::Function,
) -> Result<(), WasmError> {
    let storage = Box::new(LocalStorage::new()?);
    let anisette_provider = RemoteV3AnisetteProvider::default()?.set_storage(storage.clone());
    let mut account = AppleAccountBuilder::new(&email)
        .anisette_provider(anisette_provider)
        .login(&password, || async {
            let promise: js_sys::Promise = two_factor_callback
                .call0(&JsValue::NULL)
                .ok()?
                .dyn_into()
                .ok()?;

            let value = wasm_bindgen_futures::JsFuture::from(promise).await.ok()?;
            value.as_string()
        })
        .await?;
    console::log_1(&format!("Logged in as {}", account.email).into());
    let dev_session = DeveloperSession::from_account(&mut account).await?;

    let sideloader = SideloaderBuilder::new(dev_session, email.to_lowercase())
        .machine_name("iloader".into())
        .storage(storage)
        // .max_certs_behavior(MaxCertsBehavior::Prompt(Box::new(max_certs_callback)))
        .build();

    let sideloader_mutex = SIDELOADER.get_or_init(|| Mutex::new(None));
    *sideloader_mutex.lock().unwrap() = Some(sideloader);

    Ok(())
}

#[wasm_bindgen]
pub async fn logged_in_as() -> Result<Option<String>, WasmError> {
    let sideloader_mutex = SIDELOADER.get_or_init(|| Mutex::new(None));
    let sideloader = sideloader_mutex.lock().unwrap();
    if let Some(sideloader) = sideloader.as_ref() {
        Ok(Some(sideloader.get_email().into()))
    } else {
        Ok(None)
    }
}
