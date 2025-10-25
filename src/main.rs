use std::net::SocketAddr;

use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> listening on {addr}\n");

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, routes_hello).await.unwrap();
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
