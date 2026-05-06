use idevice::Idevice;
use idevice::provider::IdeviceProvider;
use idevice::{IdeviceError, lockdown::LockdownClient};

use netmuxd::usb::apple::{self, APPLE_VID};
use wasm_bindgen_futures::JsFuture;
use web_sys::{UsbDeviceFilter, UsbDeviceRequestOptions, console};

pub struct WebUsbProvider {
    handle: netmuxd::usb::mux::UsbMuxHandle,
    pub label: String,
}

impl std::fmt::Debug for WebUsbProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebUsbProvider").finish()
    }
}
impl WebUsbProvider {
    pub async fn new(label: &str) -> Result<Self, String> {
        request_permission().await?;

        let handle = open_mux_handle().await?;
        Ok(Self {
            handle,
            label: label.to_string(),
        })
    }
}
impl IdeviceProvider for WebUsbProvider {
    fn connect(
        &self,
        port: u16,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<idevice::Idevice, idevice::IdeviceError>> + Send>,
    > {
        let handle = self.handle.clone();
        Box::pin(async move {
            let stream = handle
                .connect(port)
                .await
                .map_err(|e| IdeviceError::UnknownErrorType(e.to_string()))?;
            Ok(idevice::Idevice::new(Box::new(stream), "iloader-next"))
        })
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn get_pairing_file(
        &self,
    ) -> std::pin::Pin<
        Box<
            dyn Future<Output = Result<idevice::pairing_file::PairingFile, idevice::IdeviceError>>
                + Send,
        >,
    > {
        let handle = self.handle.clone();
        Box::pin(async move {
            let mut lockdown = open_lockdown(&handle).await?;

            let host_id = uuid::Uuid::new_v4().to_string().to_uppercase();
            let system_buid = uuid::Uuid::new_v4().to_string().to_uppercase();
            console::log_1(&"Generated host_id={host_id} system_buid={system_buid}".into());

            console::log_1(
                &"Calling lockdown.pair() — accept the trust prompt on the device…".into(),
            );
            lockdown.pair(host_id, system_buid, None).await
        })
    }
}

async fn request_permission() -> Result<(), String> {
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
    Ok(())
}

/// Open the WebUSB device, claim the mux interface, and spawn the
/// usbmuxd-v2 task. Caller uses the returned `UsbMuxHandle` to open as many
/// virtual TCP streams as it wants — keep the handle alive for the duration
/// of all those streams.
async fn open_mux_handle() -> Result<netmuxd::usb::mux::UsbMuxHandle, String> {
    console::log_1(&"Listing devices via nusb…".into());
    let info = nusb::list_devices()
        .await
        .map_err(|e| format!("list_devices: {e}"))?
        .find(apple::is_apple_mux)
        .ok_or_else(|| {
            "no Apple usbmuxd device permitted; click Connect iPhone first".to_string()
        })?;

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
    Ok(netmuxd::usb::mux::spawn(
        1,
        serial.clone(),
        opened.reader,
        opened.writer,
        exit_tx,
    ))
}

async fn open_lockdown(
    handle: &netmuxd::usb::mux::UsbMuxHandle,
) -> Result<LockdownClient, IdeviceError> {
    console::log_1(&"Connecting virtual TCP to lockdownd port 62078…".into());
    let stream = handle
        .connect(LockdownClient::LOCKDOWND_PORT)
        .await
        .map_err(|e| IdeviceError::UnknownErrorType(e.to_string()))?;
    let idevice = Idevice::new(Box::new(stream), "iloader-web");
    Ok(LockdownClient::new(idevice))
}
