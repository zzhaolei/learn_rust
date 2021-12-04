use std::{env, net::SocketAddr, str::FromStr};

use axum::{routing::get, Router};

async fn root() -> &'static str {
    "hello world"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("R: {}", env::var("RUST_LOG").unwrap_or_default());

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from_str("127.0.0.1:8000").unwrap();
    tracing::info!("Listener {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
