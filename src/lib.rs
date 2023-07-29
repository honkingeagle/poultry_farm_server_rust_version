mod email;
mod farm;
mod user;
mod middleware;

use axum::Router;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
    smtp_email: String,
    smtp_password: String,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>, smtp_email: String, smtp_password: String) -> AppState {
        AppState {
            pool,
            smtp_email,
            smtp_password,
        }
    }
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/users", user::user_router())
        .nest("/farms", farm::farm_router(state.clone()))
        .with_state(state)
}
