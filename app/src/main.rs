
use axum::{
    routing::{get, get_service},
    Router,
    response::{Html, IntoResponse, Response}, extract::{Query, Path}, middleware
};
use model::ModelController;
use serde::Deserialize;

use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use crate::error::{Error, Result};

mod error;
mod web;
mod model;


#[tokio::main]
async fn main()  -> Result<()>{

    let mc = ModelController::new().await?;

    let router = Router::new()
        .merge(router_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_ticket::routes(mc))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();

    Ok(())

}

async fn main_response_mapper(res: Response) -> Response {
    println!("--> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/", get(get_service(ServeDir::new("./"))))
}

fn router_hello() -> Router {
    Router::new()
        .route("/foo", get(handler_hello))
        .route("/foo/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:12} - handler_handler - {params:?}", "Handler");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name:?}</strong>"))
}


async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:12} - handler_handler - {name:?}", "Handler");
    Html(format!("Hello <strong>{name:?}</strong>"))
}