use crate::web;
use crate::{Error, Result};
use axum::Json;
use axum::Router;
use axum::routing::post;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic
    if payload.username != "admin" || payload.pwd != "admin" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "token-xxx"));

    let body = Json(json!({
        "result":{
        "access_token": "token-xxx",
        "token_type": "Bearer"
        }

    }));
    Ok(body)
}
