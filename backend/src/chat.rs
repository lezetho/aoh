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

    let mut models = vec![crate::model_handler::pick_model()];
    for m in crate::model_handler::load_models() {
        if !models.contains(&m) {
            models.push(m);
        }
    }

    for model in &models {
        let body = serde_json::json!({
            "model": model,
            "messages": [{"role": "user", "content": payload.message}]
        });
        let res = match client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
        {
            Ok(r) => r,
            Err(_) => continue,
        };

        let code = res.status().as_u16();
        if code == 429 {
            continue;
        }
        if code == 404 || code == 410 {
            crate::model_handler::remove_model(model);
            continue;
        }
        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            let lower = text.to_lowercase();
            if lower.contains("discontinued") || lower.contains("decommissioned") {
                crate::model_handler::remove_model(model);
            }
            continue;
        }

        return res.text().await.map_err(|_| StatusCode::BAD_GATEWAY);
    }

    Err(StatusCode::BAD_GATEWAY)
}
