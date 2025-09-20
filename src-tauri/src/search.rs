use anyhow::{anyhow, Result};
use kuchikiki::NodeRef;
use kuchikiki::traits::TendrilSink;
use reqwest::Client;
use tauri::{AppHandle, Emitter, Manager, };
use tauri_plugin_opener::OpenerExt;

use crate::{AppState, REGION_HEIGHT, REGION_WIDTH};
use crate::ocr::{ocr};
use crate::screenshot::capture_cursor_region;

const OFFSET: f64 = 50.0;

pub const SEARCH_PAGE_WINDOW: &'static str = "search";
pub async fn search(app_handle: AppHandle) -> Result<()> {
    // Find the word under cursor
    let screen_shot = capture_cursor_region(app_handle.clone(), REGION_WIDTH, REGION_HEIGHT).await?;
    let text = ocr(app_handle.clone(), &screen_shot, REGION_WIDTH, REGION_HEIGHT).await?;

    dbg!(&text);
    let mut window_position = app_handle.cursor_position()?;

    window_position.x += OFFSET;
    window_position.y += OFFSET;

    let window = app_handle.get_webview_window(SEARCH_PAGE_WINDOW).expect("must be created during setup");
    window.set_position(window_position)?;
    window.show()?;

    // Search the word
    let client = app_handle.state::<Client>();
    let app_state = app_handle.state::<AppState>();

    let source = app_state.get_from_language();
    let target = app_state.get_to_language();

    // TODO: url encode text
    let res = client
        .get(format!("https://glosbe.com/{}/{}/{}", source, target, text))
        .send()
        .await?
        .text()
        .await?;


    let document = kuchikiki::parse_html().one(res.as_str()).document_node;

    let remove_selectors = ["button", "#translation_automatic", "#indirect-translations-container-0", ".inline-action-btn", ".fragment_expandIcon"];
    for sel in &remove_selectors {
        // parse selector and collect nodes (collect first because detaching mutates the tree)
        if let Ok(mut nodes) = document.select(sel) {
            let nodes: Vec<_> = nodes.map(|n| n.as_node().clone()).collect();
            for node in nodes {
                node.detach(); // remove from DOM
            }
        }
    }

    // Change all links to plain text
    if let Ok(mut links) = document.select("a") {
        let links: Vec<_> = links.map(|n| n.as_node().clone()).collect();
        for link in links {
            let text = link.text_contents(); // aggregated inner text
            // insert text node after the <a>, then remove the anchor
            link.insert_after(NodeRef::new_text(text));
            link.detach();
        }
    }

    let sections =
        document.select("#dictionary-content>article>div>div:nth-child(1)>section").map_err(|e| anyhow!("{:?}", e))?
            .into_iter()
            .collect::<Vec<_>>();

    let result = if sections.is_empty() {
        format!("<h1>`{}` Not Found</h1>", text)
    } else {
        sections.into_iter().map(|node| node.as_node().to_string()).collect::<Vec<_>>().join("\n")
    };



    // Display the search result on the search window
    window.emit("search-result", &result)?;


    Ok(())
}


