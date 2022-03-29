fn scrape_endpoint() -> &'static str {
    "http://localhost:9100/metrics"
}

pub async fn fetch_metrics() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let metric_payload = reqwest::get(scrape_endpoint()).await?.text().await?;

    let metrics = metric_payload
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Ok(metrics)
}
