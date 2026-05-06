mod webusb_provider;

use std::sync::{Mutex, OnceLock};

use iloader_core::read_lockdown_values;
use wasm_bindgen::prelude::*;
use webusb_provider::WebUsbProvider;

static IDEVICE: OnceLock<Mutex<Option<WebUsbProvider>>> = OnceLock::new();

#[wasm_bindgen]
pub async fn connect_idevice() -> Result<(), String> {
    let provider = WebUsbProvider::new("iloader-web").await?;

    let mutex = IDEVICE.get_or_init(|| Mutex::new(None));
    *mutex.lock().unwrap() = Some(provider);

    Ok(())
}

#[wasm_bindgen]
pub async fn read_lockdown() -> Result<String, String> {
    let mutex = IDEVICE.get_or_init(|| Mutex::new(None));
    let lock = mutex.lock().unwrap();
    let provider = lock.as_ref().ok_or("no provider")?;

    read_lockdown_values(provider).await
}
