use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: Real auth-token parsing & validation logic
    token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    Ok(next.run(req).await)
}
