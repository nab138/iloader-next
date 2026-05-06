use std::sync::Mutex;

use idevice::{
    provider::UsbmuxdProvider,
    usbmuxd::{UsbmuxdAddr, UsbmuxdConnection},
};
use iloader_core::read_lockdown_values;
use tauri::{Manager, State};

pub type ProviderMutex = Mutex<Option<UsbmuxdProvider>>;

#[tauri::command]
async fn connect_idevice(device_state: State<'_, ProviderMutex>) -> Result<(), String> {
    println!("connect_idevice called");
    let mut usbmuxd = UsbmuxdConnection::default()
        .await
        .map_err(|e| "Failed to connect to usbmuxd: ".to_string() + &e.to_string())?;

    let devs = usbmuxd
        .get_devices()
        .await
        .map_err(|e| "Failed to get devices: ".to_string() + &e.to_string())?;

    if devs.is_empty() {
        return Err("No devices found".to_string());
    }

    let usbmuxd_addr = UsbmuxdAddr::from_env_var()
        .map_err(|e| "Invalid usbmuxd address from environment: ".to_string() + &e.to_string())?;

    let dev = devs.first().unwrap();
    let provider = dev.to_provider(usbmuxd_addr, "iloader-web");
    let mut device_state = device_state.lock().unwrap();
    *device_state = Some(provider);

    Ok(())
}

#[tauri::command]
async fn read_lockdown(device_state: State<'_, ProviderMutex>) -> Result<String, String> {
    let provider = {
        let device_lock = device_state.lock().unwrap();
        match &*device_lock {
            Some(d) => d.clone(),
            None => return Err("No device connected".to_string()),
        }
    };

    read_lockdown_values(&provider).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(ProviderMutex::new(None));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![connect_idevice, read_lockdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
