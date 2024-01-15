use axum::{
    body::{Body, HttpBody},
    routing::{get, get_service, post},
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeDir};

use std::time::Duration;

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request, Response},
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info, info_span, Span};

pub mod error;
mod middleware;
pub mod routes;

pub fn create_main_rounter() -> Router {
    let x = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "request",
                method = ?request.method(),
                matched_path,
                req_body_len = tracing::field::Empty,
                res_body_len = tracing::field::Empty,
                res_time = tracing::field::Empty,
                res_status = tracing::field::Empty,
            )
        })
        .on_request(|_request: &Request<Body>, _span: &Span| {
            _span.record("req_body_len", _request.body().size_hint().lower());
        })
        .on_response(
            |_response: &Response<Body>, _latency: Duration, _span: &Span| {
                _span.record("req_body_len", _response.body().size_hint().lower());
                _span.record("res_time", format!("{:.0?}", _latency));
                _span.record("res_status", format!("{:.0?}", _response.status()));
                info!("request successfull")
            },
        )
        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
            _span.record("chunk_lengt", _chunk.len());
            // ...
        })
        .on_eos(
            |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                // ...
            },
        )
        .on_failure(
            |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                // ...
            },
        );

    let router = Router::new()
        .nest("/game", game())
        .layer(CorsLayer::permissive())
        .layer(x)
        .fallback_service(routes_static());


    router
}

fn game() -> Router {
    Router::new()
        .route("/list", get(routes::games::list))
        .route("/:id/create", post(routes::designer::create::new))
        .route("/:id/fill", post(routes::designer::fill::fill))
        .route("/:id/state", get(routes::state::state))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get(get_service(ServeDir::new("./"))))
}
