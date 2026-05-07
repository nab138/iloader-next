mod webusb;

use std::sync::{Mutex, OnceLock};

use idevice::{IdeviceService, lockdown::LockdownClient};
use iloader_core::{
    device::{ConnectionType, DeviceInfo},
    error::{AppError, WasmError},
    read_lockdown_values,
};
use netmuxd::usb::provider::UsbMuxProvider;
use wasm_bindgen::prelude::*;

use crate::webusb::get_webusb_provider;

static IDEVICE: OnceLock<Mutex<Option<UsbMuxProvider>>> = OnceLock::new();

#[wasm_bindgen]
pub async fn get_devices() -> Result<JsValue, WasmError> {
    let provider = get_webusb_provider("iloader-web").await?;

    let mut lockdown_client = LockdownClient::connect(&provider).await.map_err(|e| {
        eprintln!("Unable to connect to lockdown");
        AppError::DeviceComsWithMessage("Unable to connect to lockdown".into(), e.to_string())
    })?;

    let device_name_value = lockdown_client
        .get_value(Some("DeviceName"), None)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch DeviceName");
            AppError::DeviceComsWithMessage("Failed to fetch DeviceName".into(), e.to_string())
        })?;

    let device_name = device_name_value.as_string().ok_or_else(|| {
        eprintln!("DeviceName was not a string");
        AppError::DeviceComs("DeviceName was not a string".into())
    })?;

    let version_value = lockdown_client
        .get_value(Some("ProductVersion"), None)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch ProductVersion");
            AppError::DeviceComsWithMessage("Failed to fetch ProductVersion".into(), e.to_string())
        })?;

    let version = version_value.as_string().ok_or_else(|| {
        eprintln!("ProductVersion was not a string");
        AppError::DeviceComs("Product version was not a string".into())
    })?;

    let udid_value = lockdown_client
        .get_value(Some("UniqueDeviceID"), None)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch UniqueDeviceID");
            AppError::DeviceComsWithMessage("Failed to fetch UniqueDeviceID".into(), e.to_string())
        })?;

    let udid = udid_value.as_string().ok_or_else(|| {
        eprintln!("UniqueDeviceID was not a string");
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
pub async fn read_lockdown() -> Result<String, String> {
    let mutex = IDEVICE.get_or_init(|| Mutex::new(None));
    let lock = mutex.lock().unwrap();
    let provider = lock.as_ref().ok_or("no provider")?;

    read_lockdown_values(provider).await
}

#[wasm_bindgen]
pub async fn login(email: String, password: String) -> Result<(), String> {
    // isideload::init();
    Ok(())
}
