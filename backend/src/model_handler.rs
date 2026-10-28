use axum::{Json, http::StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct Cache {
    updated_at: u64,
    models: Vec<String>,
}

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<Model>,
}

#[derive(Deserialize)]
struct Model {
    id: String,
    #[serde(default)]
    input_modalities: Vec<String>,
    #[serde(default)]
    output_modalities: Vec<String>,
}

pub async fn sync_models() -> Result<(), Box<dyn Error>> {
    const CACHE_FILE: &str = "models.json";
    const TTL_SECS: u64 = 3 * 60 * 60;

    if let Ok(contents) = fs::read_to_string(CACHE_FILE) {
        if let Ok(cache) = serde_json::from_str::<Cache>(&contents) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            if cache.updated_at > 0 && now.saturating_sub(cache.updated_at) < TTL_SECS {
                println!("Loaded {} models", cache.models.len());
                return Ok(());
            }
        }
    }

    let api_key = std::env::var("GROQ_KEY")?;
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.groq.com/openai/v1/models")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    let parsed: ModelsResponse = res.json().await?;
    let text_models: Vec<String> = parsed
        .data
        .into_iter()
        .filter(|m| {
            m.input_modalities.iter().any(|x| x == "text")
                && m.output_modalities.iter().any(|x| x == "text")
        })
        .map(|m| m.id)
        .collect();

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let cache = Cache {
        updated_at: now,
        models: text_models.clone(),
    };
    fs::write(CACHE_FILE, serde_json::to_string_pretty(&cache)?)?;

    println!("Loaded {} models: {:#?}", text_models.len(), text_models);
    Ok(())
}

pub async fn list_models() -> (StatusCode, Json<Vec<String>>) {
    if let Ok(contents) = fs::read_to_string("models.json") {
        if let Ok(cache) = serde_json::from_str::<Cache>(&contents) {
            return (StatusCode::OK, Json(cache.models));
        }
    }
    (StatusCode::NOT_FOUND, Json(Vec::new()))
}
