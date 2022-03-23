use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Duration;

fn compute_job(job: i64) -> i64 {
    let mut rng = thread_rng();
    let sleep_ms: u64 = rng.gen_range(0..10);
    std::thread::sleep(Duration::from_millis(sleep_ms));

    job * job
}

fn process_result(result: i64) {
    println!("{}", result);
}

fn main() {
    let jobs = 0..100;

    jobs.into_par_iter()
        .map(compute_job)
        .for_each(process_result);
}
