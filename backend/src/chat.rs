use axum::{Json, http::StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChatPayload {
    pub message: String,
}

pub async fn chat(Json(payload): Json<ChatPayload>) -> Result<String, StatusCode> {
    let api_key = std::env::var("GROQ_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": "openai/gpt-oss-20b",
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