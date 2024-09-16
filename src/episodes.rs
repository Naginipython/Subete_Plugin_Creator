use quickjs_runtime::{builder::QuickJsRuntimeBuilder, jsutils::Script, values::JsValueConvertable};
use serde_json::{json, Value};

use crate::{helpers, Extras, SearchAnime};

pub async fn check_episodes(search_result: Vec<SearchAnime>) -> Extras {
    // Episodes
    let episode_url = helpers::get_url("episode");
    let episode_code = helpers::get_js("episode");
    // check for post
    let post_check: Vec<&str> = episode_url.split_ascii_whitespace().collect();
    let mut extra = Extras::new();
    let html = if post_check[0] == "POST" {
        extra.chap_extras = json!({"request": "post"});
        helpers::post_fetch(post_check[1].replace("{id}", &search_result[0].id)).await
    } else {
        helpers::fetch(episode_url.replace("{id}", &search_result[0].id)).await
    };

    println!("Testing episode...");
    let runtime = QuickJsRuntimeBuilder::new().build();
    let script = Script::new("episodes.js", &episode_code);
    runtime.eval_sync(None, script).ok().expect("script failed");
    let episode_value = runtime
        .invoke_function_sync(None, &[], "getEpisodes", vec![serde_json::to_string(&search_result[0]).unwrap().to_js_value_facade(), html.to_js_value_facade()])
        .unwrap().to_serde_value().await.unwrap();
    let mut episode_result: SearchAnime = serde_json::from_value(episode_value).unwrap_or_else(|e| {
        eprintln!("Error: Chapter script does not include all fields, or fields are wrong type; {e}");
        std::process::exit(0)
    });
    
    if let Some(e) = episode_result.extra {
        while let Some(next) = e.get("next") {
            // If there is a next link, call it
            let html = helpers::fetch(String::from(next.as_str().unwrap())).await;
            let episode_value = runtime
                .invoke_function_sync(None, &[], "next", vec![serde_json::to_string(&search_result[0]).unwrap().to_js_value_facade(), html.to_js_value_facade()])
                .unwrap().to_serde_value().await.unwrap();
            episode_result = serde_json::from_value(episode_value).unwrap_or_else(|e| {
                eprintln!("Error: Chapter script does not include all fields, or fields are wrong type; {e}");
                std::process::exit(0)
            });
            if let None = episode_result.extra { break; }
        }
    }
    println!("Episode successful");

    // lastly, video?
    let video_url = helpers::get_url("video");
    let video_code = helpers::get_js("video");

    let html = helpers::fetch(video_url.replace("{id}", &episode_result.episodes[0].id)).await;
    // let video_code2 = format!("{} getEpisodeVideo(`{html}`);", video_code);
    println!("Testing video...");
    // let video_value: Value = rustyscript::evaluate(&video_code2).expect("JS works");
    let script = Script::new("video.js", &video_code);
    runtime.eval_sync(None, script).ok().expect("script failed");
    let video_value = runtime
        .invoke_function_sync(None, &[], "getEpisodeVideo", vec![html.to_js_value_facade()])
        .unwrap().to_serde_value().await.unwrap();
    let mut video_result: Value = serde_json::from_value(video_value).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(0)
    });
    
    let mut count = 0;
    while let Some(next) = video_result.get("next") {
        count+=1;
        let next_count = if count==1 { "" } else { &format!("{}", count) };
        let html = if next.as_str().unwrap() == "BUILD" {
            // TODO: make more generic
            let iv = helpers::text_to_byte_arr(video_result["decrypt"]["iv"].as_str().unwrap());
            let key = helpers::text_to_byte_arr(video_result["decrypt"]["key"].as_str().unwrap());
            let decrypt = helpers::crypto_handler(video_result["decrypt"]["string"].as_str().unwrap(), &iv[..], &key[..], false).unwrap()
                .split('&').skip(1).collect::<Vec<_>>().join("&");
            let encrypt = helpers::crypto_handler(video_result["encrypt"]["string"].as_str().unwrap(), &iv[..], &key[..], true).unwrap();
            let link = video_result["build"]
                .as_str()
                .unwrap()
                .replace("$encrypt", &encrypt)
                .replace("$decrypt", &decrypt);
            helpers::fetch_with_header(link, "X-Requested-With", "XMLHttpRequest").await
        } else if next.as_str().unwrap() == "CRYPTO" {
            let iv = helpers::text_to_byte_arr(video_result["decrypt"]["iv"].as_str().unwrap());
            let key = helpers::text_to_byte_arr(video_result["decrypt"]["key"].as_str().unwrap());
            let decrypt = helpers::crypto_handler(video_result["decrypt"]["string"].as_str().unwrap(), &iv[..], &key[..], false).unwrap();
            video_result = json!({"data": decrypt});
            String::new()
        } else {
            helpers::fetch(String::from(next.as_str().unwrap())).await
        };

        // let video_code3 = format!(
        //     "{} next{}(JSON.parse({:?}), `{html}`);", 
        //     &video_code, 
        //     next_count,
        //     serde_json::to_string(&video_result).unwrap()
        // );
        // let video_value: Value = rustyscript::evaluate(&video_code3).expect("JS works");
        // println!("next{next_count}");
        // println!("{:?}", video_result);
        let video_value = runtime
            .invoke_function_sync(None, &[], &format!("next{next_count}"), vec![serde_json::to_string(&video_result).unwrap().to_js_value_facade(), html.to_js_value_facade()])
            .unwrap().to_serde_value().await.unwrap();
        video_result = serde_json::from_value(video_value).unwrap_or_else(|e| {
            eprintln!("Error: Chapter script does not include all fields, or fields are wrong type; {e}");
            std::process::exit(0)
        });
        // println!("{:?}", video_result);
    }
    if !video_result.as_str().unwrap().contains("m3u8") &&
        !video_result.as_str().unwrap().contains("https://") {
        eprintln!("Error: No video receieved");
        std::process::exit(0)
    }
    println!("Pages successful");

    Extras::new()
}