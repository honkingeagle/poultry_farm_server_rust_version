mod verify;
mod create_user;
mod login_user;
mod logout_user;
mod session;

use crate::{SharedState, email::generate_code};
use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoggedUser {
    id: i32,
    email: String,
}

pub fn user_router() -> Router<SharedState> {
    Router::new()
        .route("/register", post(create_user::register))
        .route("/login", post(login_user::login))
        .route("/logout", get(logout_user::logout))
        .route("/generatecode", post(generate_code::mail))
        .route("/verify", post(verify::account))
        .route("/session", get(session::validate))
}
