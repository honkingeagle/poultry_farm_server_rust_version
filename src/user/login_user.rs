use super::User;
use crate::AppState;
use axum::{
    extract::{self, State},
    http::StatusCode,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use sqlx::Row;

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    extract::Json(user): extract::Json<User>,
) -> Result<(CookieJar, StatusCode), (StatusCode, String)> {
    let query = sqlx::query("SELECT id, password, active FROM users AS u WHERE u.email = $1")
        .bind(user.email)
        .fetch_optional(&state.pool)
        .await;

    match query {
        Ok(result) => match result {
            Some(queried_user_result) => {
                let password: String = queried_user_result.get("password");
                let user_id: i32 = queried_user_result.get("id");
                let active: bool = queried_user_result.get("active");
                let verified_hash =
                    bcrypt::verify(user.password, &password).expect("Unable to verify hash");

                if active {
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
                        Err((StatusCode::BAD_REQUEST, "Incorrect password".to_string()))
                    }
                } else {
                    Err((StatusCode::BAD_REQUEST, "Please verify your email!".to_string()))
                }
            }
            None => Err((StatusCode::BAD_REQUEST, "Email not found".to_string())),
        },
        Err(err) => Err((StatusCode::BAD_REQUEST, format!("Something happened: {err}"))),
    }
}
