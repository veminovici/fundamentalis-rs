mod join_impl {

    pub enum JoinNext<J> {
        Init,
        Item(J),
        Sep,
    }

    pub struct Join<I, J> {
        xs: I,
        sep: J,
        next: JoinNext<J>,
    }

    pub fn join<I>(i: I, sep: I::Item) -> Join<I::IntoIter, I::Item>
    where
        I: IntoIterator,
    {
        Join {
            xs: i.into_iter(),
            sep,
            next: JoinNext::Init,
        }
    }

    impl<I> Iterator for Join<I, I::Item>
    where
        I: Iterator,
        I::Item: Copy,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            match self.next {
                JoinNext::Init => {
                    self.next = JoinNext::Sep;
                    self.xs.next()
                }
                JoinNext::Sep => match self.xs.next() {
                    None => None,
                    Some(x) => {
                        self.next = JoinNext::Item(x);
                        Some(self.sep)
                    }
                },
                JoinNext::Item(x) => {
                    self.next = JoinNext::Sep;
                    Some(x)
                }
            }
        }
    }
}

pub trait IteratorExt: Iterator {
    fn join(self, sep: Self::Item) -> crate::join_impl::Join<Self, Self::Item>
    where
        Self: Sized,
    {
        crate::join_impl::join(self, sep)
    }
}

impl<T> IteratorExt for T where T: Iterator + ?Sized {}


fn main() {
    println!("Hello, world!");

    let xs = [1, 2, 3];
    let a = xs.iter().map(|&x| x);
    let mut ys = a.join(0);
    assert_eq!(ys.next(), Some(1));
    assert_eq!(ys.next(), Some(0));
    assert_eq!(ys.next(), Some(2));
    assert_eq!(ys.next(), Some(0));
    assert_eq!(ys.next(), Some(3));
    assert_eq!(ys.next(), None);

}
