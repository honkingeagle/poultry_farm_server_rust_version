use crate::AppState;
use axum::{
    extract::{self, State},
    http::{StatusCode},
    response::{IntoResponse},
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Serialize, Deserialize)]
struct User {
    email: String,
    password: String,
}
async fn register(
    State(state): State<AppState>,
    extract::Json(new_user): extract::Json<User>,
) -> impl IntoResponse {
    let hashed_password = bcrypt::hash(new_user.password, 11).expect("Unable to hash password");

    let query = sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2)")
        .bind(new_user.email)
        .bind(hashed_password)
        .execute(&state.pool)
        .await;

    match query {
        Ok(_) => (StatusCode::CREATED, "Account created!".to_string()).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("Something went wrong: {e}"),
        )
            .into_response(),
    }
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    extract::Json(user): extract::Json<User>,
) -> Result<(CookieJar, StatusCode), StatusCode> {
    let query = sqlx::query("SELECT id, password FROM users AS u WHERE u.email = $1")
        .bind(user.email)
        .fetch_optional(&state.pool)
        .await;

    match query {
        Ok(result) => match result {
            Some(queried_user_result) => {
                let password: String = queried_user_result.get("password");
                let user_id: i32 = queried_user_result.get("id");
                let verified_hash =
                    bcrypt::verify(user.password, &password).expect("Unable to verify hash");

                if verified_hash {
                    let session_id: u64 = rand::random();

                    sqlx::query(
                        "INSERT INTO sessions (session_id, user_id) 
                    VALUES ($1, $2) ON CONFLICT (user_id) 
                    DO UPDATE SET session_id = EXCLUDED.session_id",
                    )
                    .bind(&session_id.to_string())
                    .bind(user_id)
                    .execute(&state.pool)
                    .await
                    .expect("Unable to insert session");

                    let cookie = Cookie::build("crusty_chicken", session_id.to_string())
                        .secure(false)
                        .same_site(SameSite::Strict)
                        .http_only(true)
                        .path("/")
                        .finish();

                    Ok((jar.add(cookie), StatusCode::OK))
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            }
            None => Err(StatusCode::BAD_REQUEST),
        },
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn logout(State(state): State<AppState>, jar: CookieJar) -> Result<CookieJar, StatusCode> {
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

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
}
