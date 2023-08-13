use super::Farm;
use crate::SharedState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::Row;

pub async fn list_farms(
    State(state): State<SharedState>,
    Path(user_id): Path<i32>,
) -> Result<(StatusCode, Json<Vec<Farm>>), (StatusCode, String)> {
    let query = sqlx::query("SELECT * FROM farms WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(&state.pool)
        .await;

    match query {
        Ok(pg_rows_farms) => {
            let pg_rows: Vec<Farm> = pg_rows_farms
                .into_iter()
                .map(|item| Farm {
                    id: item.get("id"),
                    name: item.get("name"),
                })
                .collect();
            Ok((StatusCode::OK, Json(pg_rows)))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "Resource not found".to_string())),
    }
}
