use axum::{Json, http::StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct ChatPayload {
    pub message: String,
}

#[derive(Deserialize)]
struct CachedModels {
    models: Vec<String>,
}

pub async fn chat(Json(payload): Json<ChatPayload>) -> Result<String, StatusCode> {
    let api_key = std::env::var("GROQ_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let client = reqwest::Client::new();
    let cache_file = "models.json";
    let models: Vec<String> = if let Ok(contents) = fs::read_to_string(cache_file) {
        if let Ok(cache) = serde_json::from_str::<CachedModels>(&contents) {
            cache.models
        } else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let body = serde_json::json!({
        "model": models.get(0).unwrap_or(&"openai/gpt-oss-20b".into()),
        "messages": [{"role": "user", "content": payload.message}]
    });
    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    res.text().await.map_err(|_| StatusCode::BAD_GATEWAY)
}