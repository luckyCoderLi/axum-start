use crate::{Error, Result};
use axum::Json;
use axum::Router;
use axum::routing::post;
use serde::{Deserialize, Serialize};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
#[derive(Debug, Serialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<LoginResponse>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic
    if payload.username != "admin" || payload.pwd != "admin" {
        return Err(Error::LoginFail);
    }

    let body = Json(LoginResponse {
        access_token: "token-xxx".to_string(),
        token_type: "Bearer".to_string(),
    });
    Ok(body)
}
