use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
   let http_client =  httpc_test::new_client("http://localhost:8080")?;

   http_client.do_get("/hello?name=Frank").await?.print().await?;

   Ok(())
}