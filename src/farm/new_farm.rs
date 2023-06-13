use super::Farm;
use crate::AppState;
use axum::{
    extract::{self, State},
    http::StatusCode,
    Json,
};
pub async fn create_farm(
    State(state): State<AppState>,
    extract::Json(farm_with_user_id): Json<Farm>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = sqlx::query("INSERT INTO farms (name, user_id) VALUES ($1, $2)")
        .bind(farm_with_user_id.name)
        .bind(farm_with_user_id.id)
        .execute(&state.pool)
        .await;

    match query {
        Ok(_) => Ok((StatusCode::OK, "Farm created Successfully".to_string())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            format!("Something went wrong: {e}"),
        )),
    }
}