use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use regex::Regex;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize};

const CONF_THRESHOLD: f64 = 80.0;

const LEFT_INDEX: usize = 6;
const TOP_INDEX: usize = 7;
const WIDTH_INDEX: usize = 8;
const HEIGHT_INDEX: usize = 9;
const CONF_INDEX: usize = 10;
const TEXT_INDEX: usize = 11;

pub const OVERLAY_WINDOW: &'static str = "overlay";

static REMOVE_PUNCTUATIONS_RE: OnceLock<Regex> = OnceLock::new();

pub fn remove_punctuations_and_spaces(text: &str) -> String {
    REMOVE_PUNCTUATIONS_RE
        .get_or_init(|| Regex::new(r"(?u)^[\p{P}\p{S}\s]+|[\p{P}\p{S}\s]+$").unwrap())
        .replace_all(text, "").trim().to_string()
}


// Analyzie the screenshot with given width and height
// Find the word closet to the cursor, strip leading and trailing punctuations
// And finding the lemma of the word
pub async fn ocr(app_handle: AppHandle, screenshot_path: &Path, pic_width: u32, pic_height: u32) -> Result<String> {
    let pic_center_x = pic_width as f64/ 2.0;
    let pic_center_y = pic_height as f64/ 2.0;

    let command = Command::new("tesseract")
        .arg(screenshot_path)
        .arg("stdout")
        .stderr(Stdio::null())
        .arg("tsv")
        .arg("--psm 8")
        .output()?;

    let stdout = String::from_utf8_lossy(&command.stdout);

    let mut builder = csv::ReaderBuilder::new();

    let mut rdr = builder.delimiter(b'\t').has_headers(true).from_reader(stdout.as_bytes());

    let mut candidates = Vec::new();

    for result in rdr.records() {
        let record = if let Ok(record) = result {
            record
        } else {
            continue;
        };
        let left = if let Some(left) = record.get(LEFT_INDEX).and_then(|v| v.parse::<f64>().ok()){
            left
        } else {
            continue;
        };
        let top = if let Some(top) = record.get(TOP_INDEX).and_then(|v| v.parse::<f64>().ok()){
            top
        } else {
            continue;
        };
        let width = if let Some(width) = record.get(WIDTH_INDEX).and_then(|v| v.parse::<f64>().ok()){
            width
        } else {
            continue;
        };
        let height = if let Some(height) = record.get(HEIGHT_INDEX).and_then(|v| v.parse::<f64>().ok()) {
            height
        } else {
            continue;
        };
        let conf = if let Some(conf) = record.get(CONF_INDEX).and_then(|v| v.parse::<f64>().ok()){
            conf
        } else {
            continue;
        };
        let text = if let Some(text) = record.get(TEXT_INDEX){
            text.to_string()
        } else {
            continue;
        };

        // 1. Skip records that have less than CONF_THRESHOLD
        if conf < CONF_THRESHOLD {
            continue;
        }
        // 2. Find the center of the word, and calculate distance to the screenshot center
        let x = left + width / 2.0;
        let y = top + height / 2.0;
        let dist_square = (x-pic_center_x)*(x-pic_center_x) + (y-pic_center_y)*(y-pic_center_y);
        candidates.push((text, dist_square, left, top, width, height));

    }

    candidates.sort_by(|a, b| a.1.total_cmp(&b.1));

    // 3. Find the candidate with the lowest distance
    let candidate = candidates.get(0).ok_or(
        anyhow::anyhow!("No candidate found"),
    )?;


    // 4. Clean the word for leading and trailing punctuations/spaces
    let cleaned = remove_punctuations_and_spaces(&candidate.0);

    // TODO: Make overlay work
    // let left_to_pic_center = candidate.2 - pic_center_x;
    // let top_to_pic_center = candidate.3 - pic_center_y;
    // let cursor_pos = app_handle.cursor_position()?;
    // let left = cursor_pos.x + left_to_pic_center;
    // let top = cursor_pos.y + top_to_pic_center;
    //
    // let width = candidate.4;
    // let height = candidate.5;
    //
    // if let Some(window) = app_handle.get_webview_window(OVERLAY_WINDOW) {
    //     window.set_size(PhysicalSize::new(width, height))?;
    //     window.set_position(PhysicalPosition::new(left, top))?;
    //     window.show()?;
    // }

    Ok(cleaned)
}