# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Fundamentalis...Rust...

# 1. Concurrency

- 1.1. Stream [[code](./concurrency/stream/), [resource](https://kerkour.com/rust-worker-pool)]
- 1.2. CPU Bound & Rayon [[code](./concurrency/cpubound/), [resource](https://kerkour.com/rust-worker-pool)]
- 1.3. Cron jobs using tokio [[code](./concurrency/cron-job-tokio/), [resource](https://kerkour.com/rust-background-jobs)]

# 2. IO
- 2.1. Read a file [[code](./io/file-read/), [resource](https://kerkour.com/rust-read-file)]
- 2.2. Http client [[code](./io/http-client/), [resource](https://kerkour.com/rust-small-docker-image)]
- 2.3. GRPC client and server [[code](./io/grpc-cli-srv/), [resource](https://tjtelan.com/blog/lets-build-a-single-binary-grpc-server-client-with-rust-in-2020/)]
- 2.4. GRPC [[resource-1](https://blog.logrocket.com/rust-and-grpc-a-complete-guide/), [resource-2](https://romankudryashov.com/blog/2021/04/grpc-rust/)]

# 3. Memory
- 3.1. Smart pointers [[code](./mem/smart-pointers/), [resource](https://kerkour.com/rust-avoid-lifetimes)]

#4. Utils
- 4.1. Logging [[code](./utils/logging/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-a-debug-message-to-the-console)]
- 4.2. Logging to stdoout [[code](./utils/logging-stdout/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-an-error-message-to-the-console)]
- 4.3. Logging with custom logger [[code](./utils/logging-custom/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-messages-with-a-custom-logger)]
- 4.4. Logging to syslog [[code](./utils/logging-syslog/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-to-the-unix-syslog)]

# Rust Development workflow
[resource](https://kerkour.com/rust-development-workflow)

```
cargo install cargo-audit
cargo install cargo outdated

cargo audit
cargo outdated
```
