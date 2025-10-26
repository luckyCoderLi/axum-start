use std::net::SocketAddr;

pub use self::error::{Error, Result};

use crate::model::ModelController;

use axum::{
    Router,
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    // 鉴权中间件，指定路由进行鉴权
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_hello = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> listening on {addr}\n");

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, routes_hello).await.unwrap();
    Ok(())
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

// 静态文件路由
fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// 这种函数参数写法叫做解构，Query(params) 表示提取查询参数，并将其解构为 params 变量
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> QUERY PARAMS: {params:?}");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> PATH PARAMS: {name}");
    Html(format!("Hello2 <strong>{name}</strong>"))
}
