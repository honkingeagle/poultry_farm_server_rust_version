use crate::SharedState;
use axum::{
    extract::{State, Path},
    http::StatusCode,
};

pub async fn remove_farm(
    State(state): State<SharedState>,
    Path((user_id, farm_id)): Path<(i32, i32)> 
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = sqlx::query("DELETE FROM farms WHERE user_id = $1 AND id = $2")
                .bind(user_id)
                .bind(farm_id)
                .execute(&state.pool)
                .await;

    match query {
        Ok(_) => Ok((StatusCode::OK, "Farm Deleted Successfully".to_string())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            format!("Something went wrong: {e}"),
        )),
    }
}