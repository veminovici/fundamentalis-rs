# BORROW

## Which one should I use?

We can see how they’re kind of the same: they both deal with owned and borrowed versions of some type. However, they’re a bit different.

Choose Borrow when you want to abstract over different kinds of borrowing, or when you’re building a data structure that treats owned and borrowed values in equivalent ways, such as hashing and comparison.

Choose AsRef when you want to convert something to a reference directly, and you’re writing generic code.

## When to use Borrow?
Borrow is a trait that is intended to be used for types that have a canonical way of borrowing as another type. 

A typical use case for Borrow is when you have a data that wraps around some internal type T for added functionalities. 

For example, String adds the functionality of mutability to str, so it is a superset of str, in terms of its features. In other words, String can do everything immutable that str can do. Similarly, Vec<T> is essentially a more flexible version of [T], so it again can do everything immutable that [T] can do, and so forth.

One requirement for implementing Borrow is that Hash, Eq and Ord for a borrowed value are equivalent to those of the owned value. This is the key difference from AsRef trait. If some data structure implements Borrow trait for T, then it must behave equivalently to T. Wit this requirement, the standard HashMap uses Borrow trait for get() and get_mut() methods.

## Resource

- [Borrow and AsRef](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/borrow-and-asref.html)
- [Rust tip and trick - Borrow](https://medium.com/@techhara/rust-tip-and-trick-borrow-37c8b0426a04)
- [Rust Concept Clarification: Deref vs AsRef vs Borrow vs Cow](https://dev.to/zhanghandong/rust-concept-clarification-deref-vs-asref-vs-borrow-vs-cow-13g6)