// IPv7C Soul Vision v3.0 - Backend Tauri (Rust)
// Comandos Tauri que el frontend JS puede invocar via `invoke()`

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Consulta el estado del nodo IPv7C local vía HTTP al backend Python
#[tauri::command]
async fn get_node_snapshot(port: u16) -> Result<serde_json::Value, String> {
    let url = format!("http://127.0.0.1:{}/api/snapshot", port);
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("No se pudo conectar al nodo IPv7C en puerto {}: {}", port, e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Error parseando snapshot: {}", e))?;

    Ok(json)
}

/// Envía un mensaje al chat IA del nodo IPv7C
#[tauri::command]
async fn ai_chat(port: u16, message: String) -> Result<String, String> {
    let url = format!("http://127.0.0.1:{}/api/ai/chat", port);
    let client = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("message", message);

    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Error en chat IA: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Error parseando respuesta IA: {}", e))?;

    Ok(json["reply"].as_str().unwrap_or("Sin respuesta").to_string())
}

/// Activa/desactiva el túnel VPN del nodo
#[tauri::command]
async fn toggle_node(port: u16) -> Result<bool, String> {
    let url = format!("http://127.0.0.1:{}/api/toggle", port);
    let client = reqwest::Client::new();

    let response = client
        .post(&url)
        .send()
        .await
        .map_err(|e| format!("Error toggling nodo: {}", e))?;

    Ok(response.status().is_success())
}

/// Retorna el timestamp Unix actual desde Rust
#[tauri::command]
fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_node_snapshot,
            ai_chat,
            toggle_node,
            get_timestamp
        ])
        .run(tauri::generate_context!())
        .expect("Error iniciando Soul Vision");
}
