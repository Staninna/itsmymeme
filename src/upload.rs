use axum::{extract::Multipart, http::StatusCode, response::Html};
use dotenvy::var;

pub async fn upload_page() -> Html<&'static str> {
    Html(include_str!("upload.html"))
}

pub async fn upload(mut payload: Multipart) -> Result<Html<String>, (StatusCode, &'static str)> {
    let password = match payload.next_field().await.unwrap() {
        Some(password) => password.bytes().await.unwrap_or_default(),
        None => return Err((StatusCode::BAD_REQUEST, "Password is required")),
    };

    if password != var("PASSWORD").unwrap().as_bytes() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid password"));
    }

    // TODO: Support video files
    // TODO: Support multiple files(?)
    let file = match payload.next_field().await.unwrap() {
        Some(file) => file.bytes().await.unwrap(), // Fails here with some files
        None => return Err((StatusCode::BAD_REQUEST, "File is required")),
    };

    let uuid = uuid::Uuid::new_v4();
    let ext = match infer::get(&file) {
        Some(extension) => extension,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to infer file type",
            ))
        }
    };

    let content_dir = var("CONTENT_DIR").unwrap_or_else(|_| "files".to_string());
    if !std::path::Path::new(&content_dir).exists() {
        tokio::fs::create_dir(&content_dir).await.unwrap();
    }
    tokio::fs::write(
        format!("{}/{}.{}", content_dir, uuid, ext.extension()),
        file,
    )
    .await
    .unwrap();

    let url = format!("http://localhost:3000/{}.{}", uuid, ext.extension());

    Ok(Html(format!(
        r#"<html>
                <head>
                    <title>File uploaded</title>
                </head>
                <body>
                    <p>File uploaded successfully!</p>
                    <p>URL: <a href="{}">{}</a></p>
                    <button onclick="navigator.clipboard.writeText('{}')">Copy URL</button>
                </body>
            </html>"#,
        url, url, url
    )))
}
