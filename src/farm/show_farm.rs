use super::Farm;
use crate::SharedState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::Row;
pub async fn view_farm(
    State(state): State<SharedState>,
    Path((user_id, farm_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, Json<Farm>), (StatusCode, String)> {
    let query = sqlx::query("SELECT * FROM farms WHERE user_id = $1 AND id = $2")
        .bind(user_id)
        .bind(farm_id)
        .fetch_optional(&state.pool)
        .await;

    match query {
        Ok(queried_farm_result) => match queried_farm_result {
            Some(queried_farm) => Ok((
                StatusCode::OK,
                Json(Farm {
                    id: queried_farm.get("id"),
                    name: queried_farm.get("name"),
                }),
            )),
            None => Err((
                StatusCode::BAD_REQUEST,
                "No such farm available".to_string(),
            )),
        },
        Err(e) => Err((StatusCode::BAD_REQUEST, format!("Something happened: {e}"))),
    }
}
