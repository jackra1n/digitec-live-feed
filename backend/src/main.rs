mod fetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fetch::fetch_items().await?;

    Ok(())
}
