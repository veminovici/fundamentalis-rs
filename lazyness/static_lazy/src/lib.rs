// Only night: https://rustdoc.crud.net/imperio/unify-sizes/std/lazy/struct.SyncLazy.html
// use std::lazy::SyncLazy;

// static COUNT: SyncLazy<i32> = SyncLazy::new(|| {
//     let mut count = 0;
//     count
// });

// SyncLazy<T> is Send if T is Send, so COUNT is Send since i32 is Send
