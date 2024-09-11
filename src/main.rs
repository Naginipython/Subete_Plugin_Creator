use std::{io::{self, Write}, sync::OnceLock};
use serde_json::{json, Value};
use crate::models::*;

pub static PLUGIN_DIR: OnceLock<String> = OnceLock::new();

mod models;
mod chapters;
mod episodes;
mod helpers;

#[tokio::main]
async fn main() {
    let mut query = String::from("one"); // 'mashle' is easier
    println!("Welcome to the Subete's Plugin creator!");
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 3 {
        eprintln!("Usage: {} <plugin_dir> optional:<search>", args[0]);
        std::process::exit(1);
    } else if args.len() == 2 {
        PLUGIN_DIR.get_or_init(|| args[1].clone());
    } else if args.len() == 3 {
        PLUGIN_DIR.get_or_init(|| args[1].clone());
        query = args[2].clone();
    }

    // Media type 
    let mut find_media_type: Option<MediaType> = None;
    while find_media_type.is_none() {
        print!("What is the media type of this plugin? (manga, anime, ln): ");
        let mut input: String = String::new();
        let _ = io::stdout().flush(); // allows to print then stdin
        io::stdin().read_line(&mut input).expect("Error: Expected input");
        input.pop();
        find_media_type = MediaType::from_str(&input)
    }
    let media_type = find_media_type.unwrap();

    // Search
    let search_url = helpers::get_url("search");
    let search_code = helpers::get_js("search");

    let html = helpers::fetch(search_url.replace("{title}", &query)).await;
    let search_code1 = format!("{}search(`{}`);", &search_code, &html);
    println!("Testing search...");
    let search_value: Value = rustyscript::evaluate(&search_code1).unwrap_or_else(|e1| {
        let search_code2 = format!("{}search(JSON.stringify({}));", &search_code, &html);
        rustyscript::evaluate(&search_code2).unwrap_or_else(|e2| {
            eprintln!("Error: Search script fails; {e1}");
            eprintln!("Error: Search script fails; {e2}");
            std::process::exit(0)
        })
    });
    let search_result: SearchResult = if media_type.is_manga_or_ln() {
        serde_json::from_value(search_value).map(SearchResult::MangaLn).unwrap_or_else(|e| {
            eprintln!("Error: Search script does not include all fields, or fields are wrong type; {e}");
            std::process::exit(0)
        })
    } else  {
        serde_json::from_value(search_value).map(SearchResult::Anime).unwrap_or_else(|e| {
            eprintln!("Error: Search script does not include all fields, or fields are wrong type; {e}");
            std::process::exit(0)
        })
    };
    println!("Search successful");

    let extras: Extras = match search_result {
        SearchResult::MangaLn(m) => chapters::check_chapters(m).await,
        SearchResult::Anime(a) => episodes::check_episodes(a).await,
    };

    // Creates the JSON
    if media_type.is_manga_or_ln() {
        // Setting up file
        let chap_url: String = helpers::get_url("chapter");
        let chap_url: Vec<&str> = chap_url.split_ascii_whitespace().collect();
        let chap_url = if chap_url.len() > 1 {
            chap_url[1]
        } else {
            chap_url[0]
        };
        let write: Value = json!({
            "id": PLUGIN_DIR.get().unwrap(),
            "version": "0.0.1",
            "media_type": media_type.to_str(),
            "search_url": helpers::get_url("search"),
            "search": helpers::get_js("search"),
            "search_extra": json!({}),
            "chapters_url": chap_url,
            "get_chapters": helpers::get_js("chapter"),
            "chapters_extra": extras.chap_extras,
            "pages_url": helpers::get_url("page"),
            "get_pages": helpers::get_js("page"),
            "pages_extra": json!({})
        });
        helpers::write_js(write);
    } else {
        let write: Value = json!({
            "id": PLUGIN_DIR.get().unwrap(),
            "version": "0.0.1",
            "media_type": media_type.to_str(),
            "search_url": helpers::get_url("search"),
            "search": helpers::get_js("search"),
            "search_extra": json!({}),
            "episodes_url": helpers::get_url("episode"),
            "get_episodes": helpers::get_js("episode"),
            "episodes_extra": json!({}),
            "videos_url": helpers::get_url("video"),
            "get_videos": helpers::get_js("video"),
            "videos_extra": json!({})
        });
        helpers::write_js(write);
    }
    println!("Program has completed");

}