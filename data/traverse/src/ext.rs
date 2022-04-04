use super::*;

impl<I: Traversal, O, F: FnMut(I::Item) -> O> Traversal for Map<I, F> {
    type Item = F::Output;

    fn foreach<F1>(self, mut f: F1)
    where
        F1: FnMut(F::Output) -> bool,
    {
        let mut closure = self.closure;
        self.iter.foreach(move |t| f(closure(t)));
    }
}
