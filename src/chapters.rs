use serde_json::{json, Value};

use crate::{helpers, Extras, Search};

pub async fn check_chapters(search_result: Vec<Search>) -> Extras {
    // Chapters
    let chapter_url = helpers::get_url("chapter");
    let mut chapter_code = helpers::get_js("chapter");
    // check for post
    let post_check: Vec<&str> = chapter_url.split_ascii_whitespace().collect();
    let mut extra = Extras::new();
    // anime or manga
    let html = if post_check[0] == "POST" {
        extra.chap_extras = json!({"request": "post"});
        helpers::post_fetch(post_check[1].replace("{id}", &search_result[0].id)).await
    } else {
        helpers::fetch(chapter_url.replace("{id}", &search_result[0].id)).await
    };

    chapter_code.push_str(&format!("getChapters(JSON.parse({:?}), `{html}`);", serde_json::to_string(&search_result[0]).unwrap()));
    println!("Testing chapter...");
    let chapter_value: Value = rustyscript::evaluate(&chapter_code).expect("JS works");
    let chapter_result: Search = serde_json::from_value(chapter_value).unwrap_or_else(|e| {
        eprintln!("Error: Chapter script does not include all fields, or fields are wrong type; {e}");
        std::process::exit(0)
    });
    println!("Chapter successful");

    //Pages
    let page_url = helpers::get_url("page");
    let mut page_code = helpers::get_js("page");
    
    let html = helpers::fetch(page_url.replace("{id}", &chapter_result.chapters[0].id)).await;
    page_code.push_str(&format!("getChapterPages(`{html}`);"));
    println!("Testing pages...");
    let page_value: Value = rustyscript::evaluate(&page_code).expect("JS works");
    let page_result: Vec<String> = serde_json::from_value(page_value).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(0)
    });
    if page_result.len() <= 0 {
        eprintln!("Error: No pages receieved");
        std::process::exit(0)
    }
    println!("Pages successful");
    extra 
}