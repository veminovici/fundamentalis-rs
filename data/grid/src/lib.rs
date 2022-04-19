#![feature(generic_const_exprs)]

mod bit;
mod matrix1d;
mod matrix2d;

pub use bit::*;
pub use matrix1d::*;
pub use matrix2d::*;

pub struct Index {
    /// The column in the grid
    pub col: usize,
    /// The row in the grid
    pub row: usize,
}

pub trait Matrix<T> {
    type Item;

    /// Returns the width of the grid
    fn width(&self) -> usize;

    /// Returns the height of the grid
    fn height(&self) -> usize;

    /// Returns a reference to an item
    fn get(&self, index: Index) -> &Self::Item;

    /// Set all the values in the grid.
    fn set_all<F>(&mut self, setter: F)
    where
        F: Fn(Index) -> T;

    /// Sets all the value in the grid.
    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Index) -> T,
        T: Send;
}
