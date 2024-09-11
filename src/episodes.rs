use serde_json::json;

use crate::{Extras, SearchAnime};

pub async fn check_episodes(search_results: Vec<SearchAnime>) -> Extras {
    Extras { chap_extras: json!({})}
}