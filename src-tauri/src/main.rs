// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ai_overlay_lib::routes::create_router;
use ai_overlay_lib::state::create_state;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_overlay=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let state = create_state();
    let app = create_router(state);

    // Run Axum server in a separate task (hardcoded to 0.0.0.0 for Docker)
    let addr = "0.0.0.0:3001";
    println!("Starting Axum server on {}", addr);

    let server_handle = tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // Check if we should only run the server (e.g. in Docker/headless environment)
    if std::env::var("SERVER_ONLY").unwrap_or_else(|_| "false".into()) == "true" {
        println!("SERVER_ONLY mode: Running as backend API only. Press Ctrl+C to stop.");
        let _ = server_handle.await;
    } else {
        // Run Tauri
        ai_overlay_lib::run();
    }
}
