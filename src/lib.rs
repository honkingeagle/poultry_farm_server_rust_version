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
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    Router::new()
        .nest("/users", user::user_router())
        .nest("/farms", farm::farm_router(state.clone()))
        .with_state(state)
        .layer(cors)
}
