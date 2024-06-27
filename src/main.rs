use std::{fs::File, io::{self, Read, Write}, sync::OnceLock};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

static PLUGIN_DIR: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() {
    let query = "isekai"; // 'mashle' is easier
    println!("Welcome to the Subete's Plugin creator!");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <plugin_dir>", args[0]);
        std::process::exit(1);
    } else if args.len() == 2 {
        PLUGIN_DIR.get_or_init(|| args[1].clone());
    }

    let search_url = get_url("search");
    let search_code = get_js("search");

    let html = fetch(search_url.replace("{title}", query)).await;
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
    let search_result: Vec<Search> = serde_json::from_value(search_value).unwrap_or_else(|e| {
        eprintln!("Error: Search script does not include all fields, or fields are wrong type; {e}");
        std::process::exit(0)
    });
    println!("Search successful");

    // Chapters
    let chapter_url = get_url("chapter");
    let mut chapter_code = get_js("chapter");
    // check for post
    let post_check: Vec<&str> = chapter_url.split_ascii_whitespace().collect();
    let mut is_chap_extra = json!({});
    let html = if post_check[0] == "POST" {
        is_chap_extra = json!({"request": "post"});
        post_fetch(post_check[1].replace("{id}", &search_result[0].id)).await
    } else {
        fetch(chapter_url.replace("{id}", &search_result[0].id)).await
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
    let page_url = get_url("page");
    let mut page_code = get_js("page");
    
    let html = fetch(page_url.replace("{id}", &chapter_result.chapters[0].id)).await;
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

    print!("What is the media type of this plugin? (manga, anime, ln): ");
    let mut input: String = String::new();
    let _ = io::stdout().flush(); // allows to print then stdin
    io::stdin().read_line(&mut input).expect("Error: Expected input");
    input.pop();
    if &input == "manga" || &input == "anime" || &input == "ln" {
        // Setting up file
        let chap_url: String = get_url("chapter");
        let chap_url: Vec<&str> = chap_url.split_ascii_whitespace().collect();
        let write: Value = json!({
            "id": PLUGIN_DIR.get().unwrap(),
            "media_type": input,
            "search_url": get_url("search"),
            "search": get_js("search"),
            "search_extra": json!({}),
            "chapters_url": chap_url[1],
            "get_chapters": get_js("chapter"),
            "chapters_extra": is_chap_extra,
            "pages_url": get_url("page"),
            "get_pages": get_js("page"),
            "pages_extra": json!({})
        });
        write_js(write);
        println!("Program has completed");
    } else {
        eprintln!("Error: valid media types are 'manga', 'anime', or 'ln'");
    }

}

#[derive(Serialize, Debug, Deserialize)]
struct Search {
    id: String,
    title: String,
    img: String,
    plugin: String,
    authors: String,
    artists: String,
    description: String,
    chapters: Vec<ChapterData>
}
#[derive(Serialize, Debug, Deserialize)]
struct ChapterData {
    id: String,
    number: f32,
    title: String,
    page: i32,
    completed: bool
}

fn get_js(name: &str) -> String {
    let mut file = File::open(format!("input/{}/{}.js", PLUGIN_DIR.get().unwrap(), name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.replace("\\n", " ");
    let re = regex::Regex::new(r"\s+").unwrap();
    re.replace_all(&contents, " ").to_string()
}
fn get_url(name: &str) -> String {
    let mut file = File::open(format!("input/{}/{}.txt", PLUGIN_DIR.get().unwrap(), name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
fn write_js(data: Value) {
    println!("Creating json file...");
    let json_string = serde_json::to_string_pretty(&data).unwrap();
    std::fs::create_dir_all("output").unwrap();
    let mut file = File::create(format!("output/{}.json", PLUGIN_DIR.get().unwrap())).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
}
async fn fetch(url: String) -> String {
    println!("Fetching...");
    let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .await.unwrap();
    let mut data = response.text().await.unwrap();
    data = data.replace("\n", " ").replace('`', "").replace("${", "S").replace("\\\"", "'");
    let re = regex::Regex::new(r"\s+").unwrap();
    data = re.replace_all(&data, " ").to_string();
    data
}
async fn post_fetch(url: String) -> String {
    let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
    let client = reqwest::Client::new();
    let response = client.post(url)
      .header(reqwest::header::USER_AGENT, user_agent)
      .send()
      .await.unwrap();
    let mut data = response.text().await.unwrap();
    data = data.replace("\n", " ").replace('`', "").replace("${", "S").replace("\\\"", "'");
    let re = regex::Regex::new(r"\s+").unwrap();
    data = re.replace_all(&data, " ").to_string();
    data
}