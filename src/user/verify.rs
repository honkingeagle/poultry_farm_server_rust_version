use crate::SharedState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    email: String,
    token: String
}
pub async fn account(
    State(state): State<SharedState>,
    Json(new_user): Json<NewUser>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = sqlx::query("SELECT * FROM activations WHERE email = $1 AND activation_id = $2")
        .bind(&new_user.email)
        .bind(&new_user.token)
        .fetch_optional(&state.pool)
        .await;

    match query {
        Ok(_) => {
            let query = sqlx::query("UPDATE users SET active = TRUE WHERE email = $1")
                .bind(new_user.email)
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
