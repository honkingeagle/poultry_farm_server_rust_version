use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn account(
    State(state): State<Arc<AppState>>,
    Path((email, token)): Path<(String, String)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = sqlx::query("SELECT * FROM activations WHERE email = $1 AND activation_id = $2")
        .bind(&email)
        .bind(&token)
        .fetch_optional(&state.pool)
        .await;

    match query {
        Ok(_) => {
            let query = sqlx::query("UPDATE users SET active = TRUE WHERE email = $1")
                .bind(email)
                .execute(&state.pool)
                .await;

            match query {
                Ok(_) => Ok((
                    StatusCode::OK,
                    "Your account has been activated!".to_string(),
                )),
                Err(err) => Err((StatusCode::BAD_REQUEST, format!("Something happened {err}"))),
            }
        }
        Err(_) => Err((StatusCode::FORBIDDEN, "Forbidden!".to_string())),
    }
}
