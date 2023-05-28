use axum::{routing::get, Router, extract::State };
use crate::AppState;

async fn list_farms(State(_state): State<AppState>) -> String {
    "Farms".to_string()
}

async fn view_farm(State(_state): State<AppState>) -> String {
    "Farm 1".to_string()
}

pub fn farm_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_farms))
        .route("/:id/show", get(view_farm))
}
