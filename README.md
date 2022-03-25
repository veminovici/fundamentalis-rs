# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Fundamentalis...Rust...

# 1. Concurrency

- 1.1. Stream [[code](./concurrency/stream/), [resource](https://kerkour.com/rust-worker-pool)]
- 1.2. CPU Bound & Rayon [[code](./concurrency/cpubound/), [resource](https://kerkour.com/rust-worker-pool)]
- 1.3. Cron jobs using tokio [[code](./concurrency/cron-job-tokio/), [resource](https://kerkour.com/rust-background-jobs)]

# 2. Data Structures
- 2.1. Itertools [[code](./data/iter/)]

# 3. IO
- 3.1. Read a file [[code](./io/file-read/), [resource](https://kerkour.com/rust-read-file)]
- 3.2. Http client [[code](./io/http-client/), [resource](https://kerkour.com/rust-small-docker-image)]
- 3.3. GRPC client and server [[code](./io/grpc-cli-srv/), [resource](https://tjtelan.com/blog/lets-build-a-single-binary-grpc-server-client-with-rust-in-2020/)]
- 3.4. GRPC [[resource-1](https://blog.logrocket.com/rust-and-grpc-a-complete-guide/), [resource-2](https://romankudryashov.com/blog/2021/04/grpc-rust/)]

# 4. Memory
- 4.1. Smart pointers [[code](./mem/smart-pointers/), [resource](https://kerkour.com/rust-avoid-lifetimes)]

# 5. Services
- 5.1. Static-site generator [[code](./services/static-site-generator/), [resource](https://kerkour.com/rust-static-site-generator)]

# 6. Utils
- 6.1. Logging [[code](./utils/logging/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-a-debug-message-to-the-console)]
- 6.2. Logging to stdoout [[code](./utils/logging-stdout/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-an-error-message-to-the-console)]
- 6.3. Logging with custom logger [[code](./utils/logging-custom/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-messages-with-a-custom-logger)]
- 6.4. Logging to syslog [[code](./utils/logging-syslog/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-to-the-unix-syslog)]
- 6.5. Logging other [resources](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html)
- 6.6. Open telemetry [[code](https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples/grpc)]

# 7. Others
- 7.1. Rust Development Workflow [[resource](https://kerkour.com/rust-development-workflow)]

```
cargo install cargo-audit
cargo install cargo outdated

cargo audit
cargo outdated
```
