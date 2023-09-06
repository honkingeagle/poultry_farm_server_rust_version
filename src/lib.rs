mod email;
mod farm;
mod middleware;
mod user;

use axum::{
    Router,
    http::{
        Method, 
        HeaderValue,
        header::{
            CONTENT_TYPE,
            ACCESS_CONTROL_ALLOW_HEADERS,
            ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN,
            AUTHORIZATION,
            ACCESS_CONTROL_ALLOW_CREDENTIALS
        }
    }
};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use std::sync::Arc;

pub struct AppState {
    pool: Pool<Postgres>,
    smtp_email: String,
    smtp_password: String,
    frontend_url: String 
}

impl AppState {
    pub fn new(pool: Pool<Postgres>, smtp_email: String, smtp_password: String, frontend_url: String) -> AppState {
        AppState {
            pool,
            smtp_email,
            smtp_password,
            frontend_url
        }
    }
}
type SharedState = Arc<AppState>;

pub fn create_router(state: AppState) -> Router {
    let new_state = Arc::new(state);
    let frontend_url = &new_state.frontend_url;
    let cors = CorsLayer::new()
        .allow_headers([AUTHORIZATION, ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE, ACCESS_CONTROL_ALLOW_HEADERS,ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN])
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true);

    Router::new()
        .nest("/users", user::user_router())
        .nest("/farms", farm::farm_router(Arc::clone(&new_state)))
        .with_state(Arc::clone(&new_state))
        .layer(cors)
}
