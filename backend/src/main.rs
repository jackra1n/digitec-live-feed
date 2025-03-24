mod fetch;
use fetch::fetch_feed_items;

fn main() {
    
    match fetch_feed_items() {
        Ok(items) => println!("Found {} items: {:#?}", items.len(), items),
        Err(e) => eprintln!("Request failed: {}", e),
    }
}
