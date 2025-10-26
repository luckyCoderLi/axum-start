use anyhow::Result;
use serde_json::json;
#[tokio::test]
async fn quick_dev() -> Result<()> {
    println!("->> TEST QUICK DEV");
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "pwd": "admin"
        }),
    );
    req_login.await?.print().await?;
    hc.do_get("/hello?name=John").await?.print().await?;
    hc.do_get("/hello2/Mi").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;

    hc.do_get("/api/tickets/list").await?.print().await?;
    hc.do_post(
        "/api/tickets/create",
        json!({
            "title": "test ticket"
        }),
    )
    .await?
    .print()
    .await?;
    hc.do_get("/api/tickets/list").await?.print().await?;
    hc.do_delete("/api/tickets/delete/0").await?.print().await?;
    hc.do_get("/api/tickets/list").await?.print().await?;

    Ok(())
}
