# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Fundamentalis...Rust...

# 1. Concurrency

- 1.1. Stream [[code](./concurrency/stream/), [resource](https://kerkour.com/rust-worker-pool)]
- 1.2. CPU Bound & Rayon [[code](./concurrency/cpubound/), [resource](https://kerkour.com/rust-worker-pool)]
- 1.3. Cron jobs using tokio [[code](./concurrency/cron-job-tokio/), [resource](https://kerkour.com/rust-background-jobs)]
- 1.4. Fetch metrics - htt client for node_exporter services [[code](./concurrency/fetch-metrics/)] 
- 1.4. Prometheus (tokio application, building a time-series DB, http client) [[code](./concurrency/prometheus/), [resource](https://21-lessons.com/time-series-data-and-databases-with-rust/)]

# 2. Data Structures
- 2.1. Itertools [[code](./data/iter/)]
- 2.2. Linked List - a double linked list [[code](./data/linked-list/), [resource](https://github.com/contain-rs/linked-list)]
- 2.3. Traverse [[code](./data/traverse/), [resource](https://github.com/contain-rs/rust-traverse)]
- 2.4. BList a deque [[code](./data/blist/), [resource](https://github.com/contain-rs/blist)]
- 2.5. ArrayVec using Const Generics [[code](./data/const-arrayvec/), [resource](https://adventures.michaelfbryan.com/posts/const-arrayvec/)]

# 3. IO
- 3.1. Read a file [[code](./io/file-read/), [resource](https://kerkour.com/rust-read-file)]
- 3.2. Http client [[code](./io/http-client/), [resource](https://kerkour.com/rust-small-docker-image)]
- 3.3. GRPC client and server [[code](./io/grpc-cli-srv/), [resource](https://tjtelan.com/blog/lets-build-a-single-binary-grpc-server-client-with-rust-in-2020/)]
- 3.4. GRPC [[resource-1](https://blog.logrocket.com/rust-and-grpc-a-complete-guide/), [resource-2](https://romankudryashov.com/blog/2021/04/grpc-rust/)]

# 4. Memory
- 4.1. Smart pointers [[code](./mem/smart-pointers/), [resource](https://kerkour.com/rust-avoid-lifetimes)]

# 5. Services
- 5.1. Static-site generator [[code](./services/static-site-generator/), [resource](https://kerkour.com/rust-static-site-generator)]

# 6. Logging
- 6.1. Logging [[code](./utils/logging/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-a-debug-message-to-the-console)]
- 6.2. Logging to stdoout [[code](./utils/logging-stdout/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-an-error-message-to-the-console)]
- 6.3. Logging with custom logger [[code](./utils/logging-custom/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-messages-with-a-custom-logger)]
- 6.4. Logging to syslog [[code](./utils/logging-syslog/), [resource](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-to-the-unix-syslog)]
- 6.5. Logging other [resources](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html)
- 6.6. Open telemetry [[code](https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples/grpc)]
- 6.7. Open Telemetry Instrumentation [[code](./utils/otel_instrumentation/), [resources](https://21-lessons.com/how-to-instrument-a-rust-application-with-opentelemetry/)]
- 6.8. Tracce [[code](./utils/trace/), [resource](https://21-lessons.com/getting-better-insights-into-your-rust-applications/)]

# 7. Others
- 7.1. Rust Development Workflow [[resource](https://kerkour.com/rust-development-workflow)]
- 7.2. OS type [[code](./utils/os-type/)]
- 7.3. Slice patterns [[code](./utils/slice-patterns/), [resource]https://adventures.michaelfbryan.com/posts/daily/slice-patterns/()]

```
cargo install cargo-audit
cargo install cargo outdated

cargo audit
cargo outdated
```
- 7.4 Prometheus node_exported [[resource](https://prometheus.io/docs/guides/node-exporter/)]

```
brew install node_exporter
brew services run node_exporter
curl http://localhost:9100/metrics
```

- 7.5. mdbook [[resource](https://rust-lang.github.io/mdBook/)]

# 8. VSCode Plugins
- [Rust-Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)
- [Search crates.io](https://marketplace.visualstudio.com/items?itemName=belfz.search-crates-io)
- [TOML Language](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
- [Path Intellisensee](https://marketplace.visualstudio.com/items?itemName=christian-kohler.path-intellisense)

# 9. Mac OS
- Profiles: ~/.zprofile ~/.profile

