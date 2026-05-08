use std::cell::RefCell;

use idevice::Idevice;
use idevice::pairing_file::PairingFile;
use idevice::{IdeviceError, lockdown::LockdownClient};

use iloader_core::error::AppError;
use netmuxd::usb::apple::{self, APPLE_VID};
use netmuxd::usb::mux::UsbMuxHandle;
use netmuxd::usb::provider::UsbMuxProvider;
use wasm_bindgen_futures::JsFuture;
use web_sys::{UsbDeviceFilter, UsbDeviceRequestOptions, console};

thread_local! {
    static MUX: RefCell<Option<UsbMuxHandle>> = const { RefCell::new(None) };
}

fn get_mux() -> Result<UsbMuxHandle, String> {
    MUX.with(|m| m.borrow().clone())
        .ok_or_else(|| "Select a device first".to_string())
}

pub async fn get_webusb_provider(label: &str) -> Result<UsbMuxProvider, AppError> {
    connect_iphone().await.map_err(AppError::WebUSB)?;

    let handle = get_mux().map_err(AppError::WebUSB)?;
    let pairing = pair_device(&handle, label)
        .await
        .map_err(AppError::WebUSB)?;

    Ok(UsbMuxProvider::new(handle, pairing, label.to_string()))
}

async fn connect_iphone() -> Result<(), String> {
    if MUX.with(|m| m.borrow().is_some()) {
        console::log_1(&"Mux already open. Reload the page to reconnect.".into());
        return Ok(());
    }

    let usb = web_sys::window()
        .ok_or_else(|| "no window".to_string())?
        .navigator()
        .usb();

    let filter = UsbDeviceFilter::new();
    filter.set_vendor_id(APPLE_VID);
    let filters = [filter];
    let opts = UsbDeviceRequestOptions::new(&filters);

    console::log_1(&"Requesting WebUSB device picker…".into());
    JsFuture::from(usb.request_device(&opts))
        .await
        .map_err(|e| format!("requestDevice: {e:?}"))?;
    console::log_1(&"Permission granted.".into());
    console::log_1(&"Listing devices via nusb…".into());
    let info = nusb::list_devices()
        .await
        .map_err(|e| format!("list_devices: {e}"))?
        .find(apple::is_apple_mux)
        .ok_or_else(|| "no Apple usbmuxd device permitted".to_string())?;

    console::log_1(
        &format!(
            "Found {:04x}:{:04x}  {}",
            info.vendor_id(),
            info.product_id(),
            info.serial_number().unwrap_or("(no serial)"),
        )
        .into(),
    );

    console::log_1(&"Opening device + claiming mux interface…".into());
    let opened = apple::open_mux(&info)
        .await
        .map_err(|e| format!("open_mux: {e}"))?;

    let serial = info
        .serial_number()
        .map(|s| {
            s.trim_matches(|c: char| c == '\0' || c.is_whitespace())
                .to_string()
        })
        .unwrap_or_default();

    console::log_1(&"Spawning usbmuxd-v2 mux task…".into());
    let (exit_tx, _exit_rx) = tokio::sync::oneshot::channel();
    let handle = netmuxd::usb::mux::spawn(1, serial, opened.reader, opened.writer, exit_tx);

    MUX.with(|m| *m.borrow_mut() = Some(handle));
    console::log_1(&"Mux task ready.".into());
    Ok(())
}

async fn open_lockdown(handle: &UsbMuxHandle, label: &str) -> Result<LockdownClient, IdeviceError> {
    console::log_1(&"Connecting virtual TCP to lockdownd port 62078…".into());
    let stream = handle
        .connect(LockdownClient::LOCKDOWND_PORT)
        .await
        .map_err(|e| IdeviceError::UnknownErrorType(e.to_string()))?;
    let idevice = Idevice::new(Box::new(stream), label);
    Ok(LockdownClient::new(idevice))
}

async fn pair_device(handle: &UsbMuxHandle, label: &str) -> Result<PairingFile, String> {
    let mut lockdown = open_lockdown(handle, label)
        .await
        .map_err(|e| format!("Failed to open lockdown: {e:?}"))?;

    let host_id = uuid::Uuid::new_v4().to_string().to_uppercase();
    let system_buid = uuid::Uuid::new_v4().to_string().to_uppercase();
    console::log_1(&format!("Generated host_id={host_id} system_buid={system_buid}").into());

    console::log_1(&"Calling lockdown.pair() - accept the trust prompt on the device...".into());
    let pairing_file = lockdown
        .pair(host_id, system_buid, None)
        .await
        .map_err(|e| format!("pair: {e:?}"))?;
    console::log_1(&"Pair succeeded.".into());

    Ok(pairing_file)
}
