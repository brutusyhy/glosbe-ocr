use anyhow::Result;
use tauri::{AppHandle, Manager};

use crate::{AppState, Preference};

const PREFERENCE_FILE: &'static str = "preference.json";

pub fn restore_preference(app_handle: &AppHandle) -> Result<Preference> {
    let preference_path = app_handle.path().app_config_dir()?.join(PREFERENCE_FILE);    
    
    let preference = serde_json::from_str(std::fs::read_to_string(preference_path)?.as_str())?;
    
    Ok(preference)
}

pub fn save_preference(app_handle: &AppHandle, preference: Preference) -> Result<()> {
    let preference_path = app_handle.path().app_config_dir()?.join(PREFERENCE_FILE);
    
    std::fs::write(preference_path, serde_json::to_string(&preference)?)?;
    
    Ok(())
}