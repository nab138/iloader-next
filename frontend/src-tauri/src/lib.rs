use tauri::command;

#[command]
fn transform(input: String) -> String {
    iloader_core::transform(&input)
}

#[command]
async fn proxy_request(url: String, body: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    client
        .post(&url)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![transform, proxy_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
