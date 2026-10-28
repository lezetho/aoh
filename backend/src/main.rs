mod model_sync;
mod sentry;

use axum::{Json, Router, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _sentry = sentry::init();
    tracing_subscriber::fmt::init();
    model_sync::sync_models().await.unwrap();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[derive(Serialize, Deserialize)]
struct Payload {
    ok: bool,
}

async fn root() -> (StatusCode, Json<Payload>) {
    (StatusCode::OK, Json(Payload { ok: true }))
}
