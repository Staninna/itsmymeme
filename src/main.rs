use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use dotenvy::var;
use std::net::{IpAddr, SocketAddr};

mod serve;
mod upload;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();

    let app = Router::new()
        .route("/upload", get(upload::upload_page))
        .route("/upload", post(upload::upload))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .route("/:file", get(serve::serve));

    let ip = var("IP").expect("IP not set");
    let port = var("PORT").expect("PORT not set");

    let ip: IpAddr = ip.parse().unwrap();
    let port: u16 = port.parse().unwrap();
    let addr = SocketAddr::from((ip, port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    log::info!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}
