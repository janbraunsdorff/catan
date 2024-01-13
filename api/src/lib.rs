use axum::{
    routing::{get, get_service, post},
    Router, middleware,
};
use tower_http::{services::ServeDir, cors::CorsLayer};
use tower_http::trace::TraceLayer;

pub mod routes;
pub mod error;

pub fn create_main_rounter() -> Router {
    let router = Router::new()
        .nest("/game", game())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .fallback_service(routes_static());

    router
}

fn game() -> Router {
    Router::new()
        .route("/:id/create", post(routes::create::new))
        .route("/:id/fill", post(routes::create::fill))
        .route("/:id/state", get(routes::state::state))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get(get_service(ServeDir::new("./"))))
}
