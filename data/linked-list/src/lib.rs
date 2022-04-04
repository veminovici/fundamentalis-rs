use std::{iter, mem, ptr};

//
// Structures
//

/// The node in the list.
/// It stores the element, the link to previous node, and a link to the next node.
struct Node<T> {
    /// The non-owning link to previous node.
    prev: Raw<T>,
    /// The owning link to the next node.
    next: Link<T>,
    /// The current element.
    elem: T,
}

/// A non-owning link, based on a raw pointer.
struct Raw<T> {
    ptr: *const Node<T>,
}

/// An owning link.
type Link<T> = Option<Box<Node<T>>>;

/// The double-linked list.
pub struct LinkedList<T> {
    len: usize,
    head: Link<T>,
    tail: Raw<T>,
}

//
// Implementations
//

impl<T> Raw<T> {
    /// Constructs a null reference.
    #[inline]
    fn none() -> Self {
        Raw {
            ptr: ptr::null_mut(),
        }
    }

    /// Constructs a reference to a given node.
    #[inline]
    fn some(ptr: &mut Node<T>) -> Self {
        Raw { ptr }
    }

    /// Returns the reference to an Node reference.
    #[inline]
    fn as_ref(&self) -> Option<&Node<T>> {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                Some(&*self.ptr)
            }
        }
    }

    /// Returns the reference to an mutable Node reference.
    #[inline]
    fn as_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                Some(&mut *(self.ptr as *mut Node<T>))
            }
        }
    }

    /// Takes ther reference out and nulls the current.
    #[inline]
    fn take(&mut self) -> Self {
        mem::replace(self, Raw::none())
    }

    /// Clones this reference.
    #[inline]
    fn clone(&mut self) -> Self {
        Raw { ptr: self.ptr }
    }
}

impl<T> Node<T> {
    /// Constructs a node from a given element.
    #[inline]
    fn new(elem: T) -> Self {
        Node {
            prev: Raw::none(),
            next: None,
            elem: elem,
        }
    }

    /// Links two lists.
    #[inline]
    fn link(&mut self, mut next: Box<Self>) {
        next.prev = Raw::some(self);
        self.next = Some(next);
    }

    /// Makes the given node follow the current one.
    #[inline]
    fn splice_next(&mut self, mut next: Box<Self>) {
        let mut old_next = self.next.take();
        old_next
            .as_mut()
            .map(|node| node.prev = Raw::some(&mut *next));
        next.prev = Raw::some(self);
        next.next = old_next;
        self.next = Some(next);
    }

    /// Take the next node from this one.
    #[inline]
    fn take_next(&mut self) -> Option<Box<Self>> {
        let mut next = self.next.take();
        next.as_mut().map(|node| node.prev = Raw::none());
        next
    }
}

impl<T> LinkedList<T> {
    /// Constructs an empty `LinkedList`
    #[inline]
    pub fn new() -> Self {
        LinkedList {
            len: 0,
            head: None,
            tail: Raw::none(),
        }
    }

    /// Appends at the end of the list a given element.
    pub fn push_back(&mut self, elem: T) {
        // increment the length
        self.len += 1;

        // create a new node for the element
        let mut node = Box::new(Node::new(elem));

        // set the tail to point to the newly created node
        let n = &mut *node;
        let mut old_tail = mem::replace(&mut self.tail, Raw::some(n));

        // connect the old tail to the new one
        match old_tail.as_mut() {
            // The list was empty, so the new node is also the head of the list
            None => self.head = Some(node),
            // Append this to the old tail
            Some(old_tail) => old_tail.link(node),
        }
    }

    /// Appends at the front of the list a given element.
    pub fn push_front(&mut self, elem: T) {
        // increment the length
        self.len += 1;

        // create a new node for the element
        let mut node = Box::new(Node::new(elem));
        let n = &mut *node;

        // link the new node to the old head.
        match self.head.take() {
            // list is empty, so the new node is also the new tail
            None => self.tail = Raw::some(n),
            // list was not empty
            Some(old_head) => node.link(old_head),
        }

        // set the new node as the head
        self.head = Some(node);
    }

    /// Removes the element from the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().as_mut().and_then(|tail| {
            // decrease the length of the list
            self.len -= 1;

            match tail.prev.take().as_mut() {
                // tail had no previous value, so the list only contains this node.
                // we take the node out by removing the head itself.
                None => self.head.take().map(|node| node.elem),
                // tail had a previoud value. this value will be the new tail
                Some(prev) => {
                    self.tail = Raw::some(prev);
                    prev.next.take().map(|node| node.elem)
                }
            }
        })
    }

    /// Removes the element from the head of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut head| {
            // decrease the length of the list
            self.len -= 1;

            match head.take_next() {
                // head had no next value, so we just null out the tail
                None => self.tail = Raw::none(),
                // there is a next value, which will be the new head
                Some(next) => self.head = Some(next),
            }

            head.elem
        })
    }

    /// Returns a reference to the element at the front of the list
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns a reference to the element from the back of the list
    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &node.elem)
    }

    /// Returns a mutable reference to the element at the front of the list
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    /// Returns a mutable reference to the element at the back of the list
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.as_mut().map(|node| &mut node.elem)
    }

    /// Returns the length of the list.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if the list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Removes all the elements from the list
    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.pop_front();
        }
    }

    /// Returns the forward iterator.
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: &self.head,
            tail: &self.tail,
            nelem: self.len,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear()
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elt in iter {
            self.push_back(elt);
        }
    }
}

impl<'a, T: 'a + Copy> Extend<&'a T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T> iter::FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut lst = LinkedList::default();
        lst.extend(iter);
        lst
    }
}

pub struct Iter<'a, T: 'a> {
    head: &'a Link<T>,
    tail: &'a Raw<T>,
    nelem: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.nelem == 0 {
            return None;
        }

        self.head.as_ref().map(|head| {
            self.nelem -= 1;
            self.head = &head.next;
            &head.elem
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.nelem, Some(self.nelem))
    }
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_new() {
        let lst = LinkedList::<u8>::new();
        assert_eq!(0, lst.len());
        assert!(lst.is_empty());
        assert!(lst.front().is_none());
        assert!(lst.back().is_none());
    }

    #[test]
    fn test_default() {
        let lst: LinkedList<u8> = LinkedList::default();
        assert_eq!(0, lst.len());
        assert!(lst.is_empty());
        assert!(lst.front().is_none());
        assert!(lst.back().is_none());
    }

    #[test]
    fn test_push_front() {
        let mut lst: LinkedList<u8> = LinkedList::default();
        lst.push_front(10);
        lst.push_front(20);
        lst.push_back(1);
        lst.push_back(2);

        for (idx, i) in lst.iter().enumerate() {
            match idx {
                0 => assert_eq!(20, *i),
                1 => assert_eq!(10, *i),
                2 => assert_eq!(1, *i),
                3 => assert_eq!(2, *i),
                _ => assert!(false, "Too many elements"),
            }
        }
    }

    #[test]
    fn test_extend() {
        let mut lst: LinkedList<u8> = LinkedList::default();
        lst.push_front(10);
        lst.push_front(20);

        lst.extend(vec![1, 2]);

        for (idx, i) in lst.iter().enumerate() {
            match idx {
                0 => assert_eq!(20, *i),
                1 => assert_eq!(10, *i),
                2 => assert_eq!(1, *i),
                3 => assert_eq!(2, *i),
                _ => assert!(false, "Too many elements"),
            }
        }
    }

    #[test]
    fn test_fromiterator() {
        let lst = LinkedList::from_iter(vec![20, 10, 1, 2]);

        for (idx, i) in lst.iter().enumerate() {
            match idx {
                0 => assert_eq!(20, *i),
                1 => assert_eq!(10, *i),
                2 => assert_eq!(1, *i),
                3 => assert_eq!(2, *i),
                _ => assert!(false, "Too many elements"),
            }
        }
    }
}
