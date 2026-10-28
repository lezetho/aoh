use std::error::Error;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

pub async fn sync_models() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("GROQ_KEY")?;
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.groq.com/openai/v1/models")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    println!("res status: {}", res.status());
    let body = res.text().await?;
    println!("res: {}", body);
    Ok(())
}