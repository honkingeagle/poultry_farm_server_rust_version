use super::User;
use crate::SharedState;
use axum::{
    extract::{self, State},
    http::StatusCode,
};

pub async fn register(
    State(state): State<SharedState>,
    extract::Json(new_user): extract::Json<User>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let hashed_password = bcrypt::hash(new_user.password, 11).expect("Unable to hash password");

    let query = sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2)")
        .bind(&new_user.email)
        .bind(hashed_password)
        .execute(&state.pool)
        .await;

    match query {
        Ok(_) => {
            Ok((StatusCode::CREATED, "Account created!".to_string()))
        }
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            format!("Something went wrong: {e}"),
        )),
    }
}
