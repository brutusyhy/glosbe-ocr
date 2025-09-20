use std::ptr::write;

use tauri::{AppHandle, Manager};

use crate::AppState;

#[tauri::command]
pub fn get_from_lang(
    app_handle: AppHandle
) -> String {
    app_handle.state::<AppState>().get_from_language()
}

#[tauri::command]
pub fn get_to_lang(app_handle: AppHandle) -> String {
    app_handle.state::<AppState>().get_to_language()
}

#[tauri::command]
pub fn set_from_lang(app_handle: AppHandle, lang: String) -> Result<(), String> {
    app_handle.state::<AppState>().set_from_language(&app_handle, lang).map_err(|e| e.to_string())
}

#[tauri::command]
pub  fn set_to_lang(app_handle: AppHandle, lang: String) -> Result<(), String> {
    app_handle.state::<AppState>().set_to_language(&app_handle, lang).map_err(|e| e.to_string())
}
