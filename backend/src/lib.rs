pub mod adapters;

use axum::Router;

pub fn create_app() -> Router {
    adapters::inbound::http::router()
}
