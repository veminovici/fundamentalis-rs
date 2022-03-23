use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};
use std::time::Duration;

async fn compute_job(job: i64) -> i64 {
    // Sleep for a bit
    let mut rng = thread_rng();
    let sleep_ms: u64 = rng.gen_range(0..10);
    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

    // Do the computation ...
    job * job
}

async fn process_result(result: i64) {
    println!("{}", result);
}

/// Uses `for_each_concurrent` which is the easiest way to consume `Stream`.
/// It does not return a `Stream` itself, but a `Future`, that can be `.awaited`.
async fn run_for_each_concurrency() {
    let jobs = 0..100;
    let concurrency = 43;

    stream::iter(jobs)
        .for_each_concurrent(concurrency, |job| async move {
            let result = compute_job(job).await;
            process_result(result).await;
        })
        .await;
}

async fn run_buffer_unordered() {
    let jobs = 0..100;
    let concurrency = 43;
    stream::iter(jobs)
        .map(compute_job)
        .buffer_unordered(concurrency)
        .for_each(process_result)
        .await;
}

async fn run_collect() {
    let jobs = 0..100;
    let concurrency = 43;
    let result: Vec<i64> = stream::iter(jobs)
        .map(compute_job)
        .buffer_unordered(concurrency)
        .collect()
        .await;

    println!("Res len: {:?}", result.len());
}

#[tokio::main]
async fn main() {
    //run_for_each_concurrency().await;
    //run_for_each_concurrency().await;
    run_collect().await;
}
