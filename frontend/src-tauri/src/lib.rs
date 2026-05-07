use std::sync::Mutex;

use idevice::provider::UsbmuxdProvider;
use iloader_core::read_lockdown_values;
use tauri::{Manager, State};

pub mod device;
pub type ProviderMutex = Mutex<Option<UsbmuxdProvider>>;

// macro use
use device::get_devices;

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
        .invoke_handler(tauri::generate_handler![get_devices, read_lockdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
