pub mod farm;

use axum::{routing::get, Router, extract::State};
use sqlx::{
    Pool,
    Postgres
};


#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

impl AppState{
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
async fn home(State(_state): State<AppState>) -> String {
    "Home".to_string()
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home))
        .nest("/farms", farm::farm_router())
        .with_state(state)
}
