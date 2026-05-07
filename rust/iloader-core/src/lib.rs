use idevice::{IdeviceService, lockdown::LockdownClient, provider::IdeviceProvider};

pub mod device;
pub mod error;

pub async fn read_lockdown_values(provider: &dyn IdeviceProvider) -> Result<String, String> {
    let mut lockdown = LockdownClient::connect(provider)
        .await
        .map_err(|e| format!("connect: {e:?}"))?;

    println!("Calling lockdown.get_value(None, None)…");
    let value = lockdown
        .get_value(None, None)
        .await
        .map_err(|e| format!("get_value: {e:?}"))?;

    let mut buf = Vec::new();
    plist::to_writer_xml(&mut buf, &value).map_err(|e| format!("plist serialize: {e:?}"))?;
    let xml = String::from_utf8(buf).map_err(|e| format!("utf8: {e:?}"))?;

    println!("Got {} bytes of plist:", xml.len());

    Ok(xml)
}
