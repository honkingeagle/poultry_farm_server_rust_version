use crate::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    Json
};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
}

pub async fn validate(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let poultry_farm_cookie = jar
        .get("crusty_chicken")
        .map(|cookie| cookie.value().to_owned());

    match poultry_farm_cookie {
        Some(cookie) => {
            let query_sessions = sqlx::query("SELECT * FROM sessions WHERE session_id = $1")
                .bind(cookie)
                .fetch_optional(&state.pool)
                .await;

            match query_sessions {
                Ok(result) => match result {
                    Some(queried_result) => {
                        let user_id: i32 = queried_result.get("user_id");  
                        Ok((StatusCode::OK, Json(User {id: user_id})))                   
                    },
                    None => Err(StatusCode::UNAUTHORIZED)
                },
                Err(_) => Err(StatusCode::BAD_REQUEST)      
            }
        }
        None => Err(StatusCode::BAD_REQUEST),
    }    
}