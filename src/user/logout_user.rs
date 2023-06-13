use crate::AppState;
use axum::{extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<CookieJar, StatusCode> {
    let poultry_farm_cookie = jar
        .get("crusty_chicken")
        .map(|cookie| cookie.value().to_owned());

    match poultry_farm_cookie {
        Some(cookie) => {
            let query = sqlx::query("DELETE FROM sessions WHERE session_id = $1")
                .bind(cookie)
                .execute(&state.pool)
                .await;
            match query {
                Ok(_) => Ok(jar.remove(Cookie::named("crusty_chicken"))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        None => Err(StatusCode::BAD_REQUEST),
    }
}
