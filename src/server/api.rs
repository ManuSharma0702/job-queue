use std::error::Error;
use axum::{routing::get, Router};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(handle_root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Job Queue running on 127.0.0.1:8080...");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_root() -> &'static str {
    "Hello job queue"
}
