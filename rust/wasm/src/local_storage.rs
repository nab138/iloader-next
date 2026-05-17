use iloader_core::error::AppError;
use isideload::util::storage::SideloadingStorage;
use rootcause::prelude::*;

#[derive(Clone)]
pub struct LocalStorage {
    storage: web_sys::Storage,
}

impl LocalStorage {
    pub fn new() -> Result<Self, Report> {
        Ok(Self {
            storage: web_sys::window()
                .ok_or_else(|| report!("Unable to access browser window"))?
                .local_storage()
                .map_err(|e| {
                    AppError::LocalStorage(e.as_string().unwrap_or("Unknown JS Error".to_string()))
                })
                .context("Unable to access local storage")?
                .ok_or_else(|| report!("Local storage is not available"))?,
        })
    }
}

impl SideloadingStorage for LocalStorage {
    fn store(&self, key: &str, value: &str) -> Result<(), Report> {
        self.storage
            .set_item(key, value)
            .map_err(|e| {
                AppError::LocalStorage(e.as_string().unwrap_or("Unknown JS Error".to_string()))
            })
            .context("Failed to store item in local storage")?;
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<Option<String>, Report> {
        let res = self
            .storage
            .get_item(key)
            .map_err(|e| {
                AppError::LocalStorage(e.as_string().unwrap_or("Unknown JS Error".to_string()))
            })
            .context("Failed to retrieve item from local storage")?;
        Ok(res)
    }
}
