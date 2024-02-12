use std::marker::PhantomData;

fn add_two(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    iter.recursive_chain(|iter| match (iter.next(), iter.next()) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    })
}

trait IteratorExt: Iterator {
    fn recursive_chain<F, R>(self, f: F) -> RecursiveChain<Self, F, R>
    where
        Self: Sized,
        F: FnMut(&mut Self) -> Option<R>,
    {
        RecursiveChain {
            base: self,
            f,
            r: PhantomData::default(),
        }
    }
}

impl<I: Iterator> IteratorExt for I {}

struct RecursiveChain<B, F, R> {
    base: B,
    f: F,
    r: PhantomData<R>,
}

impl<B: Iterator, F: FnMut(&mut B) -> Option<R>, R> Iterator for RecursiveChain<B, F, R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.base)
    }
}

fn main() {
    let iter = 0..1_000_000_000;
    // consume iter and generate an iterator that returns sum of pairs
    let sum_iter = add_two(iter); // [0+1, 2+3, 4+5, 6+7, ...]
    println!("{}", sum_iter.count());
}
