use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
   let http_client =  httpc_test::new_client("http://localhost:8080")?;

   http_client.do_get("/hello?name=Frank").await?.print().await?;

   let req_login = http_client.do_post("/api/login", json!({
      "username": "demo1",
      "password": "welcome"
   }));
   req_login.await?.print().await?;

   Ok(())
}