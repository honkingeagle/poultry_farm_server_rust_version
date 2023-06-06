use crate::AppState;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::extract::cookie::CookieJar;

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
async fn list_farms(State(_state): State<AppState>) -> String {
    "Farms".to_string()
}

async fn view_farm(State(_state): State<AppState>) -> String {
    "Farm 1".to_string()
}

pub fn farm_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_farms))
        .route("/:id", get(view_farm))
        .route_layer(middleware::from_fn_with_state(
            state,
            validate_session_middleware,
        ))
}
