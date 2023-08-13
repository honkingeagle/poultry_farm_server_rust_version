mod email;
mod farm;
mod middleware;
mod user;

use axum::{
    Router,
    http::Method
};
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;

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
type SharedState = Arc<AppState>;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    let new_state = Arc::new(state);
    Router::new()
        .nest("/users", user::user_router())
        .nest("/farms", farm::farm_router(Arc::clone(&new_state)))
        .with_state(Arc::clone(&new_state))
        .layer(cors)
}
