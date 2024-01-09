use crate::api::state::state;
use axum::{
    routing::{get, get_service, post},
    Router,
};
use tower_http::services::ServeDir;

mod init;
mod model;
pub mod state;

pub fn create_main_rounter() -> Router {
    let router = Router::new()
        .nest("/game", game())
        .fallback_service(routes_static());
    router
}

fn game() -> Router {
    Router::new()
        .route("/:id/new", post(init::new))
        .route("/:id/board", post(init::fill))
        .route("/:id/state", get(state))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get(get_service(ServeDir::new("./"))))
}
