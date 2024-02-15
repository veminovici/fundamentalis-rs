#![feature(coroutines, coroutine_trait)]
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn main() {
    let a: i32 = 4;
    let mut coroutine = move || {
        println!("Hello");
        yield a * 2;
        println!("world!");
    };

    match Pin::new(&mut coroutine).resume(()) {
        CoroutineState::Yielded(8) => {
            println!("We got the yielded value")
        }
        _ => {
            println!("Something went wrong!");
            panic!("unexpected return from resume");
        }
    }

    match Pin::new(&mut coroutine).resume(()) {
        CoroutineState::Complete(()) => {
            println!("We got the returned value");
        }
        _ => panic!("unexpected return from resume"),
    }
}
