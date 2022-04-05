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
    pub fn new() -> ArrayVec<T, { N }> {
        unsafe {
            ArrayVec {
                items: MaybeUninit::uninit().assume_init(),
                length: 0,
            }
        }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.length
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_length: usize) {
        debug_assert!(new_length <= self.capacity());
        self.length = new_length;
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn capacity(&self) -> usize {
        N
    }

    pub const fn remaining_capacity(&self) -> usize {
        self.capacity() - self.len()
    }

    pub const fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    pub fn as_ptr(&self) -> *const T {
        self.items.as_ptr() as *const T
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.items.as_mut_ptr() as *mut T
    }

    pub unsafe fn push_unchecked(&mut self, item: T) {
        debug_assert!(!self.is_full());

        let len = self.len();

        // index into the underlying array using pointer arithmetic
        // and write the item to the correct spot.
        self.as_mut_ptr().add(len).write(item);

        self.set_len(len + 1);
    }

    pub fn push(&mut self, item: T) {
        match self.try_push(item) {
            Ok(_) => {}
            Err(e) => panic!("Push failed: {}", e),
        }
    }

    pub fn try_push(&mut self, item: T) -> Result<(), CapacityError<T>> {
        if self.is_full() {
            Err(CapacityError(item))
        } else {
            unsafe {
                self.push_unchecked(item);
                Ok(())
            }
        }
    }
}

pub struct CapacityError<T>(pub T);

impl<T> core::fmt::Display for CapacityError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Insufficient capacity")
    }
}
