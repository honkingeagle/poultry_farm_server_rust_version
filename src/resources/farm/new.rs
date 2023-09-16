use crate::SharedState;
use axum::{
    extract::{self, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct NewFarm {
    id: i32,
    name: String,
}
pub async fn create_farm(
    State(state): State<SharedState>,
    extract::Json(farm_with_user_id): Json<NewFarm>,
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
