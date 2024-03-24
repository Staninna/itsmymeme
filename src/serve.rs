use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use dotenvy::var;

#[cfg(not(debug_assertions))]
const DISCORD_USER_AGENT: [&str; 2] = ["Discordbot/2.0", "Intel Mac OS X 11.6"];

pub async fn serve(Path(path): Path<String>, headers: HeaderMap) -> impl IntoResponse {
    match headers.get("User-Agent") {
        #[cfg(debug_assertions)]
        Some(_) => serve_file(path).await.into_response(),
        #[cfg(not(debug_assertions))]
        Some(user_agent) if DISCORD_USER_AGENT.contains(&user_agent.to_str().unwrap()) => {
            serve_file(path).await.into_response()
        }
        _ => forbidden().into_response(),
    }
}

async fn serve_file(path: String) -> impl IntoResponse {
    let content_dir = var("CONTENT_DIR").unwrap_or_else(|_| "files".to_string());
    let file_path = format!("{}/{}", content_dir, path);
    match tokio::fs::read(&file_path).await {
        Ok(file) => {
            let content_type = infer::get(&file)
                .map(|ext| format!("{}", ext))
                .unwrap_or_else(|| "application/octet-stream".to_string());

            let headers = [(header::CONTENT_TYPE, content_type)];

            Ok((StatusCode::OK, headers, file))
        }
        Err(_) => Err(not_found()),
    }
}

fn forbidden() -> impl IntoResponse {
    (
        StatusCode::FORBIDDEN,
        var("FORBIDDEN").unwrap_or_else(|_| "403 Forbidden".to_string()),
    )
}

fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        var("NOT_FOUND").unwrap_or_else(|_| "404 Not Found".to_string()),
    )
}
