use std::alloc::{self, Layout};
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>, // pointer to the first element in the vector
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {

            let t = unsafe {
                &*self.ptr.as_ptr().add(index)
            };

            Some(t)
        }
    }

    pub fn push(&mut self, item: T) {
        if std::mem::size_of::<T>() == 0 {
            panic!("No zero sized type");
        }

        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Could not allocate the new vector");
            // SAFETY: the layout is hardcoded to be 4 * size_of<T>()
            // size_of<T> is > 0
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not allocate memory");
            // SAFETY: ptr is not-null and we have just allocated enoug space for this items (and 3 more).
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Arithmetic overflow");
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            // SAFETY: offset cannot wrap around
            unsafe { self.ptr.as_ptr().add(self.len).write(item) };
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);

            let new_capacity = self.capacity.checked_mul(2).expect("Cannot multiple by 2");
            let size = std::mem::size_of::<T>() * self.capacity;

            unsafe {
                let layout = Layout::from_size_align_unchecked(size, std::mem::align_of::<T>());
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size) as *mut T;
                self.ptr = NonNull::new(ptr).expect("Could not allocate memory");
                self.ptr.as_ptr().add(self.len).write(item);
            }

            self.capacity = new_capacity;
            self.len += 1;
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.capacity == 0 {
        } else {
            unsafe {
                std::ptr::drop_in_place(std::slice::from_raw_parts_mut(
                    self.ptr.as_ptr(),
                    self.len,
                ));
                let layout = Layout::from_size_align_unchecked(
                    std::mem::size_of::<T>() * self.capacity,
                    std::mem::align_of::<T>(),
                );
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uninitialized() {
        let v = MyVec::<usize>::new();
        assert_eq!(v.capacity(), 0);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn first_grow() {
        let mut v = MyVec::<usize>::new();
        v.push(1);

        assert_eq!(v.capacity(), 4);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn add_two_items() {
        let mut v = MyVec::<usize>::new();
        v.push(1);
        v.push(2);

        assert_eq!(v.capacity(), 4);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn push_items() {
        let mut vec = MyVec::<usize>::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn push_get_items() {
        let mut vec = MyVec::<usize>::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);

        let r = vec.get(1);
        assert!(r.is_some());
        assert_eq!(*r.unwrap(), 2);
    }

}
