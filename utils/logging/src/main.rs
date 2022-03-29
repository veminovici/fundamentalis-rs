// RUST_LOG=debug cargo run -- bin logging
// cargo run --bin logging

fn execute_query(query: &str) {
    log::debug!("Executing query:{}", query);
}

fn main() {
    env_logger::init();
    execute_query("INSERT");
}
