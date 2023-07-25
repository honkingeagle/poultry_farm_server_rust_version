mod farm;
mod user;
mod validate_session;

use axum::Router;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

impl From<Pool<Postgres>> for AppState {
    fn from(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/users", user::user_router())
        .nest("/farms", farm::farm_router(state.clone()))
        .with_state(state)
}
