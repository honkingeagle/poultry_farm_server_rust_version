pub mod farm;

use axum::{Router};
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

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/farms", farm::farm_router())
        .with_state(state)
}