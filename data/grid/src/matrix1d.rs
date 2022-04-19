use super::*;

/// Implements a grid using an array of arrays.
pub struct Matrix1D<T, const W: usize, const H: usize>
where
    [(); W * H]: Sized,
{
    items: [T; W * H],
}

impl<T, const W: usize, const H: usize> Default for Matrix1D<T, W, H>
where
    [(); W * H]: Sized,
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            items: [T::default(); W * H],
        }
    }
}

impl<T, const W: usize, const H: usize> Matrix<T> for Matrix1D<T, W, H>
where
    [(); W * H]: Sized,
{
    type Item = T;

    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }

    fn get(&self, index: Index) -> &Self::Item {
        &self.items[index.col * W + index.row]
    }

    /// Set all the values in the grid.
    fn set_all<F>(&mut self, setter: F)
    where
        F: Fn(Index) -> T,
    {
        self.items.iter_mut().enumerate().for_each(|(i, item)| {
            let (row, col) = (i / W, i % W);
            let index = Index { col, row };
            *item = setter(index);
        });
    }

    /// Sets all the value in the grid.
    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Index) -> T,
        T: Send,
    {
        use rayon::prelude::*;
        self.items.par_iter_mut().enumerate().for_each(|(i, item)| {
            let (row, col) = (i / W, i % W);
            let index = Index { col, row };
            *item = setter(index);
        });
    }
}
