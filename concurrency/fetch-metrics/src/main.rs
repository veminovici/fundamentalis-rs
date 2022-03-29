fn scrape_endpoint() -> &'static str {
    "http://localhost:9100/metrics"
}

async fn fetch_metrics() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let metric_payload = reqwest::get(scrape_endpoint())
        .await?
        .text()
        .await?;

    let metrics = metric_payload.lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Ok(metrics)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let fetcher = tokio::spawn(async move {
        let results = fetch_metrics().await.unwrap();
        println!("Result: {:?}", results);
    });

    fetcher.await.unwrap();

    Ok(())
}
