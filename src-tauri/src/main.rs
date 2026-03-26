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
    // Check for required environment variables at startup
    if std::env::var("GEMINI_API_KEY").is_err() {
        eprintln!(
            "Error: GEMINI_API_KEY is not set. The server will not be able to process AI requests."
        );
        eprintln!("Please set it in your environment or .env file.");
    }

    let state = create_state();
    let app = create_router(state);

    // Get port from environment or default to 3001
    let port = std::env::var("API_PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Bind to the address first to catch errors early
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))
        .expect("Server failed to start");

    println!("==========================================");
    println!("🚀 Server starting on: http://{}", addr);
    println!("📖 Swagger docs at:   http://{}/docs", addr);
    println!("==========================================");

    // Run Axum server in a separate task
    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Check if we should only run the server (e.g. in Docker/headless environment)
    if std::env::var("SERVER_ONLY").unwrap_or_else(|_| "false".into()) == "true" {
        println!("[INFO] SERVER_ONLY mode: Backend API is now active.");
        let _ = server_handle.await;
    } else {
        // Run Tauri (Desktop mode)
        ai_overlay_lib::run();
    }
}
