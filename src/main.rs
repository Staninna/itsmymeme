use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

mod serve;
mod upload;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = Router::new()
        .route("/upload", get(upload::upload_page))
        .route("/upload", post(upload::upload))
        .route("/:file", get(serve::serve));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    println!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}
