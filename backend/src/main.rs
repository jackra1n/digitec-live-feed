mod fetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = fetch::fetch_items().await?;
    println!("{:?}", items);

    Ok(())
}
