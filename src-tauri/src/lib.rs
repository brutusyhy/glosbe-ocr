mod search;
mod screenshot;
mod ocr;

mod commands;
mod preferences;

use anyhow::Result;
use std::path::PathBuf;
use std::sync::{RwLock};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use crate::ocr::OVERLAY_WINDOW;
use crate::preferences::{restore_preference, save_preference};
use crate::search::{search, SEARCH_PAGE_WINDOW};

const REGION_WIDTH: u32 = 300;
const REGION_HEIGHT: u32 = 150;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preference {
    pub from_language: String,
    pub to_language: String,
}

pub struct AppState {
    pub pref: RwLock<Preference>,
}

impl AppState {
    pub fn get_from_language(&self) -> String{
        self.pref.read().unwrap().from_language.clone()
    }
    pub fn get_to_language(&self) -> String {
        self.pref.read().unwrap().to_language.clone()
    }
    pub fn set_from_language(&self, app_handle: &AppHandle, new_lang: String) -> Result<()> {
        let mut pref = self.pref.write().unwrap();
        pref.from_language = new_lang;
        save_preference(app_handle, (*pref).clone())
    }
    pub fn set_to_language(&self, app_handle: &AppHandle, new_lang: String) -> Result<()> {
        let mut pref = self.pref.write().unwrap();
        pref.to_language = new_lang;
        save_preference(app_handle, (*pref).clone())
    }

}

const MAIN_WINDOW: &str = "glosbe-ocr";
pub async fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit",true, None::<&str> )?;

            let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .build(app)?;

            let app_handle = app.app_handle();
            // Create SearchResult window but not display it immediately
            let search_window = tauri::WebviewWindowBuilder::new(
                app_handle,
                SEARCH_PAGE_WINDOW,
                tauri::WebviewUrl::App(PathBuf::from("/search-result"))
            ).visible(false).build()?;

            let overlay_window = tauri::WebviewWindowBuilder::new(
                app_handle,
                OVERLAY_WINDOW,
                tauri::WebviewUrl::App(PathBuf::from("/overlay")),
            ).decorations(false)
                .transparent(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .visible(false)
                .build()?;

            search_window.set_always_on_top(true)?;

            dbg!(app_handle.path().app_data_dir().unwrap());
            std::fs::create_dir_all(app_handle.path().app_config_dir().unwrap());

            // Read preference or use fallback default
            let init_pref = if let Ok(preference) = restore_preference(app_handle) {
                preference
            } else {
                Preference {
                    from_language: "nl".to_string(),
                    to_language: "en".to_string()
                }
            };

            app.manage::<AppState>(AppState {
                pref: RwLock::new(init_pref),
            });
            
            Ok(())
        })
        .on_window_event(|window, event| match event{
            tauri::WindowEvent::CloseRequested {api, ..} => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "settings" => {
                app.get_webview_window("main").unwrap().show().unwrap()
            }
            _ => {

            }
        })
        .plugin({
            use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
            let ctrl_d_shortcut = Shortcut::new(Some(Modifiers::CONTROL),Code::KeyD);
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(vec![ctrl_d_shortcut.clone()]).unwrap()
                .with_handler(move |app, shortcut, event | {
                if shortcut == &ctrl_d_shortcut && event.state == ShortcutState::Pressed {
                    if let Err(e) = futures::executor::block_on(search(app.app_handle().to_owned())) {
                        dbg!(e);
                    }
                }
            }).build()

        })
        .plugin({
            tauri_plugin_opener::Builder::new().build()
        })
        .manage(reqwest::Client::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_from_lang,
            commands::get_to_lang,
            commands::set_from_lang,
            commands::set_to_lang,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
