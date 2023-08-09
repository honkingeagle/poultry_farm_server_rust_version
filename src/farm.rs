mod all_farms;
mod new_farm;
mod show_farm;

use crate::{middleware::session, AppState};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Farm {
    id: i32,
    name: String,
}

pub fn farm_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/:user_id", get(all_farms::list_farms))
        .route("/:user_id/farm/:farm_id", get(show_farm::view_farm))
        .route("/new", post(new_farm::create_farm))
        .route_layer(middleware::from_fn_with_state(state, session::validate))
}
