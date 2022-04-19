use super::*;

/// Implements a grid using an array of arrays.
pub struct Matrix2D<T, const W: usize, const H: usize> {
    items: [[T; W]; H],
}

impl<T, const W: usize, const H: usize> Default for Matrix2D<T, W, H>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            items: [[T::default(); W]; H],
        }
    }
}

impl<T, const W: usize, const H: usize> Matrix<T> for Matrix2D<T, W, H> {
    type Item = T;

    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }

    fn get(&self, index: Index) -> &Self::Item {
        &self.items[index.col][index.row]
    }

    /// Set all the values in the grid.
    fn set_all<F>(&mut self, setter: F)
    where
        F: Fn(Index) -> T,
    {
        self.items.iter_mut().enumerate().for_each(|(y, row)| {
            for (x, item) in row.iter_mut().enumerate() {
                let index = Index { col: x, row: y };
                *item = setter(index);
            }
        });
    }

    /// Sets all the value in the grid.
    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Index) -> T,
        T: Send,
    {
        use rayon::prelude::*;
        self.items.par_iter_mut().enumerate().for_each(|(y, row)| {
            for (x, item) in row.iter_mut().enumerate() {
                let index = Index { col: x, row: y };
                *item = setter(index);
            }
        });
    }
}
