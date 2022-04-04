pub trait Traversal: Sized {
    /// The type of the items.
    type Item;

    /// Run this Iterator using the provided closure.
    ///
    /// Return true from the closure to end the iteration.
    fn foreach<F>(self, f: F)
    where
        F: FnMut(Self::Item) -> bool;

    fn run<F>(self, mut f: F)
    where
        F: FnMut(Self::Item),
    {
        self.foreach(|elem| {
            f(elem);
            false
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn map<F, O>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> O,
    {
        Map {
            iter: self,
            closure: f,
        }
    }
}

pub trait IntoTraversal {
    /// The intotrav type
    type IntoTrav: Traversal<Item = Self::Item>;

    /// the type of the traversal item
    type Item;

    /// convert the self into a traversal.
    fn into_traversal(self) -> Self::IntoTrav;
}

pub trait FromTraversal<T> {
    fn from_traversal<I: IntoTraversal<Item = T>>(traversable: I) -> Self;
}

impl<T: Traversal> IntoTraversal for T {
    type IntoTrav = Self;
    type Item = <Self as Traversal>::Item;

    fn into_traversal(self) -> Self::IntoTrav {
        self
    }
}

#[derive(Copy, Clone)]
pub struct Map<I, F> {
    iter: I,
    closure: F,
}

mod ext;
