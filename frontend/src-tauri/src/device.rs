use idevice::{
    IdeviceService,
    lockdown::LockdownClient,
    usbmuxd::{Connection, UsbmuxdAddr, UsbmuxdConnection},
};
use iloader_core::device::{ConnectionType, DeviceInfo};
use iloader_core::error::AppError;

pub async fn get_usbmuxd() -> Result<UsbmuxdConnection, AppError> {
    UsbmuxdConnection::default()
        .await
        .map_err(|e| AppError::Usbmuxd("Failed to connect to usbmuxd".into(), e.to_string()))
}

#[tauri::command]
pub async fn get_devices() -> Result<Vec<Result<DeviceInfo, AppError>>, AppError> {
    let mut usbmuxd = get_usbmuxd().await?;

    let devs = usbmuxd.get_devices().await.map_err(|e| {
        AppError::Usbmuxd("Failed to list devices from usbmuxd".into(), e.to_string())
    })?;
    if devs.is_empty() {
        return Ok(vec![]);
    }

    let usbmuxd_addr = UsbmuxdAddr::from_env_var().map_err(|e| {
        AppError::Usbmuxd(
            "Invalid usbmuxd address from environment".into(),
            e.to_string(),
        )
    })?;

    let device_info_futures: Vec<_> = devs
        .iter()
        .map(|d| {
            let usbmuxd_addr = usbmuxd_addr.clone();
            async move {
                let provider = d.to_provider(usbmuxd_addr, "iloader");
                let connection_type = match d.connection_type {
                    Connection::Usb => ConnectionType::USB,
                    Connection::Network(_) => ConnectionType::Network,
                    Connection::Unknown(_) => ConnectionType::Unknown,
                };

                let mut lockdown_client =
                    LockdownClient::connect(&provider).await.map_err(|e| {
                        eprintln!("Unable to connect to lockdown for {}: {e:?}", d.udid);
                        AppError::DeviceComsWithMessage(
                            "Unable to connect to lockdown".into(),
                            e.to_string(),
                        )
                    })?;

                let device_name_value = lockdown_client
                    .get_value(Some("DeviceName"), None)
                    .await
                    .map_err(|e| {
                    eprintln!("Failed to fetch DeviceName for {}: {e:?}", d.udid);
                    AppError::DeviceComsWithMessage(
                        "Failed to fetch DeviceName".into(),
                        e.to_string(),
                    )
                })?;

                let device_name = device_name_value.as_string().ok_or_else(|| {
                    eprintln!("DeviceName for {} was not a string", d.udid);
                    AppError::DeviceComs("DeviceName was not a string".into())
                })?;

                let version_value = lockdown_client
                    .get_value(Some("ProductVersion"), None)
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to fetch ProductVersion for {}: {e:?}", d.udid);
                        AppError::DeviceComsWithMessage(
                            "Failed to fetch ProductVersion".into(),
                            e.to_string(),
                        )
                    })?;

                let version = version_value.as_string().ok_or_else(|| {
                    eprintln!("ProductVersion for {} was not a string", d.udid);
                    AppError::DeviceComs("Product version was not a string".into())
                })?;

                Ok::<DeviceInfo, AppError>(DeviceInfo {
                    name: device_name.to_string(),
                    udid: d.udid.clone(),
                    connection_type,
                    version: version.to_string(),
                })
            }
        })
        .collect();

    let device_infos = futures::future::join_all(device_info_futures).await;
    Ok(device_infos)
}
