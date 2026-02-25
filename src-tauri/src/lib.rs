// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use std::sync::Mutex;

// State to track if overlay is currently visible
struct OverlayState {
    is_visible: Mutex<bool>,
}

// Command to toggle overlay visibility
#[tauri::command]
async fn toggle_overlay(
    app: tauri::AppHandle,
    state: tauri::State<'_, OverlayState>,
) -> Result<bool, String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    let mut is_visible = state.is_visible.lock().unwrap();
    *is_visible = !*is_visible;
    
    if *is_visible {
        // Show window and disable click-through
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        set_click_through_internal(&window, false)?;
    } else {
        // Hide window
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(*is_visible)
}

// Command to set click-through mode
#[tauri::command]
async fn set_click_through(
    app: tauri::AppHandle,
    enabled: bool,
) -> Result<(), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    set_click_through_internal(&window, enabled)
}

// Internal function to set click-through using Windows API
fn set_click_through_internal(
    window: &tauri::WebviewWindow,
    enabled: bool,
) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE,
            WS_EX_LAYERED, WS_EX_TRANSPARENT,
        };
        
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        let hwnd = HWND(hwnd.0);
        
        unsafe {
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            
            let new_style = if enabled {
                // Enable click-through
                ex_style | (WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0) as isize
            } else {
                // Disable click-through
                ex_style & !(WS_EX_TRANSPARENT.0 as isize)
            };
            
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);
        }
    }
    
    Ok(())
}

// Structs for Gemini API interaction
#[derive(serde::Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(serde::Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(serde::Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(serde::Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    error: Option<GeminiError>,
}

#[derive(serde::Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContentResponse>,
}

#[derive(serde::Deserialize)]
struct GeminiContentResponse {
    parts: Option<Vec<GeminiPartResponse>>,
}

#[derive(serde::Deserialize)]
struct GeminiPartResponse {
    text: Option<String>,
}

#[derive(serde::Deserialize)]
struct GeminiError {
    message: String,
}

#[derive(serde::Deserialize)]
struct ModelsResponse {
    models: Option<Vec<ModelInfo>>,
}

#[derive(serde::Deserialize)]
struct ModelInfo {
    name: String, // e.g. "models/gemini-1.5-flash"
    #[serde(rename = "supportedGenerationMethods")]
    supported_generation_methods: Option<Vec<String>>,
}

async fn get_best_available_model(api_key: &str) -> Result<(String, String), String> {
    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models?key={}", api_key);
    
    // Try listing models
    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !res.status().is_success() {
        // If listing fails, fallback to safe default
        println!("Failed to list models (status {}). Using fallback.", res.status());
        return Ok(("gemini-pro".to_string(), "v1".to_string()));
    }

    let data: ModelsResponse = res.json().await.map_err(|e| e.to_string())?;
    
    if let Some(models) = data.models {
        // Strategy: Look for best models in order of preference
        // We look for models that support "generateContent"
        let candidates: Vec<String> = models.into_iter()
            .filter(|m| m.supported_generation_methods.as_ref()
                .map_or(false, |methods| methods.contains(&"generateContent".to_string())))
            .map(|m| m.name.replace("models/", ""))
            .collect();

        println!("Available Models: {:?}", candidates);
            
        // Preference list - Back to 1.5 Flash as 2.0/2.5 have 0 quota for this key
        let preferences = [
            "gemini-1.5-flash-latest",
            "gemini-1.5-flash",
            "gemini-1.5-flash-001",
            "gemini-1.5-flash-002",
            "gemini-flash-latest" 
        ];
        
        for pref in preferences {
            // Check for exact match or versioned match
            if let Some(found) = candidates.iter().find(|c| c.starts_with(pref)) {
                println!("Auto-discovered model: {}", found);
                return Ok((found.clone(), "v1beta".to_string()));
            }
        }
    }
    
    // Absolute fallback - default to standard Flash if discovery misses
    println!("Discovery failed. Fallback to gemini-1.5-flash");
    Ok(("gemini-1.5-flash".to_string(), "v1beta".to_string()))
}

// Command to send request to AI
#[tauri::command]
async fn send_to_ai(prompt: String, model: String) -> Result<String, String> {
    // Check for API Key
    let api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY environment variable not set. Please check your .env file.".to_string())?;
    let api_key = api_key.trim(); 

    // Dynamic Discovery: Get the best available model for this key
    let (model_name, api_version) = if model == "gemini" {
        get_best_available_model(api_key).await?
    } else {
        // Allow manual override if they specifically asked for something else
        (model, "v1beta".to_string())
    };

    println!("Using Auto-Selected Model: {} (API Version: {})", model_name, api_version);

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/{}/models/{}:generateContent?key={}",
        api_version, model_name, api_key
    );

    // Debug log
    println!("Requesting URL: https://generativelanguage.googleapis.com/{}/models/{}:generateContent?key=MASKED", api_version, model_name);

    let request_body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart {
                text: prompt,
            }],
        }],
    };

    let res = client.post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Network request failed: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let error_text = res.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
        return Err(format!("API Error ({}): {}", status, error_text));
    }

    let response_json: GeminiResponse = res.json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(error) = response_json.error {
        return Err(format!("Gemini API Error: {}", error.message));
    }

    if let Some(candidates) = response_json.candidates {
        if let Some(candidate) = candidates.first() {
            if let Some(content) = &candidate.content {
                if let Some(parts) = &content.parts {
                    if let Some(part) = parts.first() {
                        if let Some(text) = &part.text {
                            return Ok(text.clone());
                        }
                    }
                }
            }
        }
    }

    Err("No valid response text found in API response.".to_string())
}

// Command to hide overlay (for close button)
#[tauri::command]
async fn hide_overlay(
    app: tauri::AppHandle,
    state: tauri::State<'_, OverlayState>,
) -> Result<(), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    let mut is_visible = state.is_visible.lock().unwrap();
    *is_visible = false;
    
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(OverlayState {
            is_visible: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            toggle_overlay,
            set_click_through,
            send_to_ai,
            hide_overlay,
        ])
        .setup(|app| {
            // Get the main window
            let window = app.get_webview_window("main").unwrap();

            // Native blurred background removed to allow reading text behind
            // #[cfg(target_os = "windows")]
            // {
            //    use window_vibrancy::apply_acrylic;
            //    let _ = apply_acrylic(&window, Some((0, 0, 0, 10)));
            // }

            // Prevent window from being captured by screen sharing/recording
            #[cfg(target_os = "windows")]
            {
                use windows::Win32::UI::WindowsAndMessaging::{SetWindowDisplayAffinity, WDA_EXCLUDEFROMCAPTURE};
                use windows::Win32::Foundation::HWND;
                if let Ok(hwnd) = window.hwnd() {
                    unsafe {
                        let _ = SetWindowDisplayAffinity(HWND(hwnd.0), WDA_EXCLUDEFROMCAPTURE);
                    }
                }
            }
            
            // Set window size to 70% width and 80% height of screen
            if let Ok(monitor) = window.primary_monitor() {
                if let Some(monitor) = monitor {
                    let size = monitor.size();
                    let width = (size.width as f64 * 0.7) as u32;
                    let height = (size.height as f64 * 0.8) as u32;
                    
                    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                        width,
                        height,
                    }));
                    
                    // Center the window
                    let _ = window.center();
                }
            }
            
            // Register global shortcut (Ctrl + Space)
            let app_handle = app.handle().clone();
            
            app.global_shortcut().on_shortcut("Ctrl+Space", move |_app, _shortcut, event| {
                if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                    let app_clone = app_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        let state = app_clone.state::<OverlayState>();
                        let app_for_toggle = app_clone.clone();
                        let _ = toggle_overlay(app_for_toggle, state).await;
                    });
                }
            })?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
