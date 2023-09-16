mod all;
mod new;
mod delete;

use crate::{
    // middleware::session, 
    SharedState
};
use axum::{
    // middleware,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Farms {
    id: i32,
    name: String,
    males: i32,
    females: i32
}
pub fn farm_router(_state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/:user_id", get(all::list_farms))
        .route("/new", post(new::create_farm))
        .route("/:user_id/delete/:farm_id", post(delete::remove_farm))
        // .route_layer(middleware::from_fn_with_state(state, session::validate))
}
