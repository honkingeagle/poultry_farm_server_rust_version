mod all_farms;
mod show_farm;
mod new_farm;

use crate::{AppState, validate_session};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};



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
            validate_session::validate_session_middleware,
        ))
}