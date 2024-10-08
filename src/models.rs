use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, PartialEq)]
pub enum MediaType {
    Manga, Ln, Anime
}
impl MediaType {
    pub fn from_str(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "manga" | "m" => Some(MediaType::Manga),
            "ln" | "l" => Some(MediaType::Ln),
            "anime" | "a" => Some(MediaType::Anime),
            _ => None
        }
    }
    pub fn is_manga_or_ln(&self) -> bool {
        match self {
            MediaType::Ln | MediaType::Manga => true,
            _ => false
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            MediaType::Anime => "anime",
            MediaType::Ln => "ln",
            MediaType::Manga => "manga"
        }
    }
}

#[derive(Debug)]
pub enum SearchResult {
    MangaLn(Vec<Search>),
    Anime(Vec<SearchAnime>)
}
#[derive(Serialize, Debug, Deserialize)]
pub struct Search {
    pub id: String,
    pub title: String,
    pub img: String,
    pub plugin: String,
    pub authors: String,
    pub artists: String,
    pub description: String,
    pub chapters: Vec<ChapterData>
}
#[derive(Serialize, Debug, Deserialize)]
pub struct ChapterData {
    pub id: String,
    pub number: f32,
    pub title: String,
    pub page: i32,
    pub completed: bool
}
#[derive(Serialize, Debug, Deserialize)]
pub struct SearchAnime {
    pub id: String,
    pub title: String,
    pub img: String,
    pub plugin: String,
    pub studio: String,
    pub status: String,
    pub description: String,
    pub episodes: Vec<EpisodeData>,
    pub extra: Option<Value>
}
#[derive(Serialize, Debug, Deserialize)]
pub struct EpisodeData {
    pub id: String,
    pub number: f32,
    pub title: String,
    pub watch_time: i64,
    pub completed: bool
}

#[derive(Debug)]
pub struct Extras {
    pub chap_extras: Value
}
impl Extras {
    pub fn new() -> Self {
        Extras {
            chap_extras: json!({})
        }
    }
}