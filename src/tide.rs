
#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/health").get(|_| async { Ok("ok") });
    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
