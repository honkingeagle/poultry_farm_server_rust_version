mod activate;
mod create_user;
mod login_user;
mod logout_user;

use crate::SharedState;
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

pub fn user_router() -> Router<SharedState> {
    Router::new()
        .route("/register", post(create_user::register))
        .route("/login", post(login_user::login))
        .route("/logout", get(logout_user::logout))
        .route("/activate/:email/:token", post(activate::account))
}
