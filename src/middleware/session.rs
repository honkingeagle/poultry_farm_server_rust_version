use crate::AppState;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;

pub async fn validate<B>(
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let poultry_farm_cookie = jar
        .get("crusty_chicken")
        .map(|cookie| cookie.value().to_owned());

    match poultry_farm_cookie {
        Some(cookie) => {
            let query_sessions = sqlx::query("SELECT * FROM sessions WHERE session_id = $1")
                .bind(cookie)
                .execute(&state.pool)
                .await;

            match query_sessions {
                Ok(_) => next.run(request).await,
                Err(_) => (
                    StatusCode::FORBIDDEN,
                    "Forbidden! Nooon-sense👀".to_string(),
                )
                    .into_response(),
            }
        }
        None => (
            StatusCode::FORBIDDEN,
            "Forbidden! Nooon-sense👀".to_string(),
        )
            .into_response(),
    }
}
