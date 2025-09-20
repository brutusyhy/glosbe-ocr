use std::path::PathBuf;
use anyhow::{anyhow, Result};
use tauri::{AppHandle, Manager};
use xcap::image::RgbaImage;
use xcap::Monitor;

pub async fn capture_cursor_region(app_handle: AppHandle, width: u32, height: u32) -> Result<PathBuf> {
    let cursor_position = app_handle.cursor_position()?;

    // TODO: Support multiple monitors?
    let monitor = Monitor::all()?.first().ok_or(anyhow!("no monitor found"))?.to_owned();

    // Clamp the location x, y to be larger than 0
    let x = if cursor_position.x as u32 > width/2 {
        cursor_position.x as u32 - width/2
    } else {
        0
    };

    let y = if cursor_position.y as u32 > height/2 {
        cursor_position.y as u32 - height/2
    } else {
        0
    };


    let image = monitor.capture_region(x, y, width, height)?;

    let path = app_handle.path().temp_dir()?.join("screenshot.png");


    image.save(&path)?;

    Ok(path)
}