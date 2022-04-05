#![no_std]
#![feature(adt_const_params)]
#![allow(incomplete_features)]

use core::mem::MaybeUninit;

/// A vector type backed by a fixed-length array.
pub struct ArrayVec<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    length: usize,
}

impl<T, const N: usize> ArrayVec<T, { N }> {
    #[inline]
    pub const fn len(&self) -> usize {
        self.length
    }
}
