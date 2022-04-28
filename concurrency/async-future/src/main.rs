use std::{cmp::min, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let text = reqwest::get("http://example.org").await?.text().await?;
    println!("response = {}", &text[..min(text.len(), 40)]);

    Ok(())
}
