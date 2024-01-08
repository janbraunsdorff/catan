use crate::{web, Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("--> {:<12} - api_login", "HANDLER");

    if payload.username != "jan" || payload.pwd != "test" {
        return Err(Error::LogingFail);
    }

    let c = Cookie::build((web::AUTH_TOKEN, "user-1.exp.sign"))
        .http_only(true)
        .build();

    cookies.add(c);

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Deserialize, Debug)]
struct LoginPayload {
    username: String,
    pwd: String,
}
