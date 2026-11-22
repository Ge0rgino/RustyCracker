use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> &'static str {
    "pong"
}
