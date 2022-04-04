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

impl<I: Traversal, F: FnMut(&I::Item) -> bool> Traversal for Filter<I, F> {
    type Item = I::Item;

    fn foreach<F1>(self, mut f: F1)
    where
        F1: FnMut(I::Item) -> bool,
    {
        let mut predicate = self.predicate;
        self.iter
            .foreach(move |t| if predicate(&t) { f(t) } else { false });
    }
}
