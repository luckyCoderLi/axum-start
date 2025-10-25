
## 版本更新

### 1. axum 从 0.6到0.8版本
> Path写法变更

``` rust
/hello2/:name -> /hello2/{name}
```

> fallback_service 取代 nest_service

``` rust
-- old
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

-- new
fn routes_static() -> Router {
    Router::new().fallback_service (get_service(ServeDir::new("./")))
}
```

## trait 相关

1. 使用 Cookies 时，返回响应体不能适用自定义结构体，否则报错

   ``` rust
   /**
    *  报错信息如下，trait bound 问题，自定义响应体有未实现的 trait
    error[E0277]: the trait bound `fn(Cookies, Json<LoginPayload>) -> ... {api_login}: Handler<_, _>` is not satisfied
    --> src/web/routes_login.rs:11:44
        |
    11 |     Router::new().route("/api/login", post(api_login))
        |                                       ---- ^^^^^^^^^ the trait `Handler<_, _>` is not implemented for fn item `fn(Cookies, Json<LoginPayload>) -> ... {api_login}`
        |                                       |
        |                                       required by a bound introduced by this call
        |
        = note: Consider using `#[axum::debug_handler]` to improve the error message
        = help: the following other types implement trait `Handler<T, S>`:
                `Layered<L, H, T, S>` implements `Handler<T, S>`
                `MethodRouter<S>` implements `Handler<(), S>`
    note: required by a bound in `post`
    */
    use axum::routing::post;
    use serde::Deserialize;
    use serde_json::Value;
    use serde_json::json;
    use tower_cookies::Cookie;
    use tower_cookies::Cookies;

   #[derive(Debug, Deserialize)]
    struct LoginPayload {
        username: String,
        pwd: String,
    }

    // 错误写法
    #[derive(Debug, Serialize)]
    struct LoginResponse {
        access_token: String,
        token_type: String,
    }
    async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<LoginResponse>> {
    }

    // 正确写法
    async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    }

   ```
2. 
