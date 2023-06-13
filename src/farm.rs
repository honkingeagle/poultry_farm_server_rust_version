mod all_farms;
mod show_farm;
mod new_farm;

use crate::AppState;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

async fn validate_session_middleware<B>(
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Farm {
    id: i32,
    name: String,
}

pub fn farm_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/:user_id", get(all_farms::list_farms))
        .route("/:user_id/farm/:farm_id", get(show_farm::view_farm))
        .route("/new", post(new_farm::create_farm))
        .route_layer(middleware::from_fn_with_state(
            state,
            validate_session_middleware,
        ))
}