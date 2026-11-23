pub mod adapters;
pub mod domain;

use axum::Router;

pub fn create_app() -> Router {
    adapters::inbound::http::router()
}
