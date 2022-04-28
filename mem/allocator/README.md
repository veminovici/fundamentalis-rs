## Run

cargo build && ../../target/debug/allocator sample --lib std 2>! events.ldjson && ../../target/debug/allocator report events.ldjson

cargo build && ../../target/debug/allocator sample --lib smol 2>! events.ldjson && ../../target/debug/allocator report events.ldjson

cargo build && ../../target/debug/allocator sample --lib smart 2>! events.ldjson && ../../target/debug/allocator report events.ldjson
