use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri_plugin_store;
use crate::tauri_plugin_store::StoreBuilder;

#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

#[tauri::command]
async fn login(username: String, password: String, app_handle: tauri::AppHandle) -> Result<LoginResponse, String> {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8080/token")
        .form(&[
            ("username", &username),
            ("password", &password),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let login_response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        save_tokens(&app_handle, &login_response.access_token, &login_response.refresh_token)?;
        Ok(login_response)
    } else {
        let error_response: ErrorResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(error_response.error)
    }
}

#[tauri::command]
async fn logout(app_handle: tauri::AppHandle) -> Result<(), String> {
    clear_tokens(&app_handle)?;
    Ok(())
}

#[tauri::command]
async fn refresh_token(app_handle: tauri::AppHandle) -> Result<LoginResponse, String> {
    let store = StoreBuilder::new(&app_handle, std::path::PathBuf::from("tokens.json")).build().map_err(|e| e.to_string())?;
    let refresh_token = store.get("refresh_token")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .ok_or("No refresh token found".to_string())?;

    let client = Client::new();
    let response = client.post("http://127.0.0.1:8080/refresh")
        .header("Authorization", format!("Bearer {}", refresh_token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let login_response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        save_tokens(&app_handle, &login_response.access_token, &login_response.refresh_token)?;
        Ok(login_response)
    } else {
        let error_response: ErrorResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(error_response.error)
    }
}

fn save_tokens(app_handle: &tauri::AppHandle, access_token: &str, refresh_token: &str) -> Result<(), String> {
    let store = StoreBuilder::new(app_handle, std::path::PathBuf::from("tokens.json"))
        .build()
        .map_err(|e| e.to_string())?;
    
    store.set("access_token", access_token.to_string());
    store.set("refresh_token", refresh_token.to_string());
    
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}


fn clear_tokens(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let store = StoreBuilder::new(app_handle, std::path::PathBuf::from("tokens.json")).build().map_err(|e| e.to_string())?;
    store.delete("access_token");
    store.delete("refresh_token");
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![login, logout, refresh_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}