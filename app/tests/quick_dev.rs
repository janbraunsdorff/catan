use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/foo?name=Jan").await?.print().await?;

    hc.do_get("/foo/Jan").await?.print().await?;

    //hc.do_get("/app/src/main.rs").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "jan",
            "pwd": "test1"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "jan",
            "pwd": "test"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post("/api/tickets", json!({"title": "Hello World"}))
        .await?
        .print()
        .await?;

    hc.do_get("/api/tickets").await?.print().await?;

    hc.do_delete("/api/tickets/0").await?.print().await?;

    Ok(())
}
