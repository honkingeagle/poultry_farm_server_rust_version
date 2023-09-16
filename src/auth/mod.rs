mod verify;
mod register;
mod login;
mod logout;
mod session;

use login::login;
use register::register;
use logout::logout;

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
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/generatecode", post(generate_code::mail))
        .route("/verify", post(verify::account))
        .route("/session", get(session::validate))
}
