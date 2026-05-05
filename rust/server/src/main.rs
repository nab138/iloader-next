use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::any,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let app = Router::new()
        .route("/proxy/{*path}", any(proxy_handler))
        .layer(CorsLayer::permissive())
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("Server listening on http://127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn proxy_handler(
    Path(path): Path<String>,
    State(client): State<reqwest::Client>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    let url = format!("https://{}", path);
    match client.post(&url).headers(headers).body(body).send().await {
        Ok(res) => {
            let status = StatusCode::from_u16(res.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            let body = res.text().await.unwrap_or_default();
            (status, body).into_response()
        }
        Err(e) => (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
    }
}
