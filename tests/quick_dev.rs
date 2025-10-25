use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    println!("->> TEST QUICK DEV");
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=John").await?.print().await?;
    hc.do_get("/hello2/Mi").await?.print().await?;
    Ok(())
}
