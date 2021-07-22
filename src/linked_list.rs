use std::{
    iter::FromIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index},
    usize,
};

use crate::prelude::{Order, Sortable};

fn last_mut<T>(list: &mut LinkedList<T>) -> Option<&mut LinkedList<T>> {
    fn assert_failed<T>() -> T {
        panic!("Cannot find last of an empty list");
    }

    if list.is_empty() {
        assert_failed()
    } else {
        let mut last = list;
        while last.tail().is_some() {
            last = last.tail_mut().unwrap();
        }

        Some(last)
    }
}

fn for_each_until_last<T, F: Fn(&mut LinkedList<T>)>(
    list: &mut LinkedList<T>,
    f: F,
) -> Option<&mut LinkedList<T>> {
    fn assert_failed<T>() -> T {
        panic!("Cannot find last of an empty list");
    }

    if list.is_empty() {
        assert_failed()
    } else {
        let mut last = list;
        f(last);
        while last.tail().is_some() {
            match last.tail_mut() {
                Some(_) => {
                    last = last.tail_mut().unwrap();
                    f(last);
                },
                None => break
            }
        }

        Some(last)
    }
}

/// An implementation of a linked list that supports empty sets.
///
/// Note that accessing the `value` of an empty list will result in
/// undefined behaviour.
///
/// ## Example
/// ```
/// # use fluffy_structs::LinkedList;
/// let mut list = LinkedList::new("Hello");
/// list.append(" world");
/// assert_eq!(list.len(), 2);
/// list.iter().for_each(|s| println!("{}", s));
/// ```
pub struct LinkedList<T: Sized> {
    value: T,
    next: Option<*const LinkedList<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    /// Initializes an empty list
    pub fn empty() -> Self {
        let assumed_init_value;
        unsafe {
            assumed_init_value = std::mem::zeroed::<T>();
        }
        Self {
            value: assumed_init_value,
            next: None,
            size: 0,
        }
    }

    /// Creates a new list with its head set to `value`
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            next: None,
            size: 1,
        }
    }

    pub fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    pub fn as_ptr_mut(&mut self) -> *mut Self {
        self as *mut Self
    }

    /// The head of the list
    pub fn head(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.value)
        }
    }

    /// Returns `true` if the list is empty
    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// The size of the list. For any given node in a list, `len()`
    /// represents the size of the subsequent list include the item
    /// it is called on.
    pub fn len(&self) -> usize {
        self.size
    }

    /// All of the list but its first item (its head)
    pub fn tail(&self) -> Option<&LinkedList<T>> {
        if self.is_empty() {
            None
        } else {
            match &self.next {
                Some(next_ptr) => unsafe { Some(&**next_ptr) },
                None => None,
            }
        }
    }

    /// Same as `tail` but returns a mutable reference to the tail of the list
    pub fn tail_mut(&mut self) -> Option<&mut LinkedList<T>> {
        if self.is_empty() {
            None
        } else {
            match self.next {
                Some(boxed_next) => unsafe { Some(&mut *(boxed_next as *mut LinkedList<T>)) },
                None => None,
            }
        }
    }

    /// The last element of the list
    pub fn last(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            let mut last = self;
            while last.tail().is_some() {
                match last.tail() {
                    Some(boxed_next) => {
                        last = boxed_next;
                    }
                    None => break,
                }
            }

            Some(last)
        }
    }

    /// Return the element at index `idx`, if any
    pub fn nth(&self, idx: usize) -> Option<&T> {
        self.iter().nth(idx)
    }

    /// Returns a mutable reference to the last item in the list
    pub fn last_mut(&mut self) -> Option<&mut LinkedList<T>> {
        if self.is_empty() {
            None
        } else {
            last_mut(self)
        }
    }

    /// Appends `value` to the list
    pub fn append(&mut self, value: T) {
        if self.is_empty() {
            self.value = value;
            self.size += 1;
        } else {
            let last = for_each_until_last(
                self, 
                |i| { i.size += 1; }
            ).unwrap();
            // let last = last_mut(self).unwrap();
            let layout = std::alloc::Layout::new::<LinkedList<T>>();
            let new_list;
            unsafe {
                new_list = std::alloc::alloc(layout);
                std::ptr::write(new_list as *mut LinkedList<T>, LinkedList::new(value));
            }
            last.next = Some(new_list as *const LinkedList<T>);
        }

        // self.size += 1;
    }

    /// Removes the elements at position `index`
    pub fn remove_at(&mut self, index: usize) {
        let hole = self.remove_at_in(index);
        drop(hole);
    }

    fn remove_at_in(&mut self, index: usize) -> Option<*const LinkedList<T>> {
        fn assert_failed<T>(idx: usize) -> T {
            panic!(
                "Attempted to remove an item with index {}, which is out of this list bounds",
                idx
            );
        }

        let mut i = 0usize;
        let mut indirect = self as *mut LinkedList<T>;
        let mut prev = std::ptr::null::<LinkedList<T>>();
        if index > self.len() {
            return assert_failed(index);
        }

        while i < index {
            let deref_indirect;
            unsafe {
                deref_indirect = &mut *indirect;
            }
            match deref_indirect.next {
                Some(next) => {
                    deref_indirect.size -= 1;
                    prev = indirect;
                    indirect = next as *mut LinkedList<T>;
                }
                None => {
                    break;
                }
            }
            i += 1;
        }

        if i != index {
            // End reached before index
            assert_failed(index)
        } else {
            let deref_prev;
            unsafe {
                deref_prev = &mut *(prev as *mut LinkedList<T>);
            }

            let new_next;
            unsafe {
                new_next = (*indirect).next.unwrap();
            }

            deref_prev.next = Some(new_next);
            // self.size -= 1;

            Some(indirect)
        }
    }

    /// Returns an iterator on the list
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator::new(self)
    }

    /// Same as `iter()`, but mutable
    pub fn iter_mut(&mut self) -> LinkedListIteratorMut<T> {
        LinkedListIteratorMut::new(self)
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list: LinkedList<T> = LinkedList::empty();

        for i in iter {
            list.append(i);
        }

        list
    }
}

impl<G: Iterator> From<G> for LinkedList<<G as Iterator>::Item>
where
    <G as Iterator>::Item: PartialOrd + Clone,
{
    fn from(iterable: G) -> Self {
        let mut list: LinkedList<<G as Iterator>::Item> = LinkedList::empty();
        for i in iterable {
            list.append(i);
        }
        list
    }
}

impl<T> Deref for LinkedList<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for LinkedList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> AsMut<T> for LinkedList<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.nth(index).as_ref().unwrap()
    }
}

// List iterators

/// An iterator over a `LinkedList`
pub struct LinkedListIterator<'a, T> {
    ptr: Option<&'a LinkedList<T>>,
}

impl<'a, T> LinkedListIterator<'a, T> {
    pub fn new(list: &'a LinkedList<T>) -> Self {
        if list.is_empty() {
            Self { ptr: None }
        } else {
            Self { ptr: Some(list) }
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = None;
        if self.ptr.is_some() {
            value = self.ptr;
            self.ptr = self.ptr.unwrap().tail();
        }

        match value {
            Some(v) => Some(&v.value),
            None => None,
        }
    }
}

pub struct LinkedListIteratorMut<'a, T> {
    ptr: *mut LinkedList<T>,
    marker: PhantomData<&'a T>,
}

impl<'a, T> LinkedListIteratorMut<'a, T> {
    pub fn new(list: &mut LinkedList<T>) -> Self {
        if list.is_empty() {
            Self {
                ptr: std::ptr::null_mut(),
                marker: PhantomData,
            }
        } else {
            Self {
                ptr: list,
                marker: PhantomData,
            }
        }
    }
}

impl<'a, T> Iterator for LinkedListIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value: *mut LinkedList<T> = std::ptr::null_mut();
        if !self.ptr.is_null() {
            unsafe {
                value = self.ptr;
                match self.ptr.as_mut().unwrap().tail_mut() {
                    Some(next_mut) => {
                        self.ptr = next_mut;
                    }
                    None => {
                        self.ptr = std::ptr::null_mut();
                    }
                }
            }
        }

        if value.is_null() {
            None
        } else {
            unsafe { Some(&mut *value) }
        }
    }
}

impl<T: Ord + Copy> Sortable for LinkedList<T> {
    fn sort(self, order: crate::prelude::Order) -> Self {
        let mut v = self.iter().map(|i| *i).collect::<Vec<T>>();
        v.sort();

        if order == Order::Descending {
            v.reverse();
        }

        let mut list: LinkedList<T> = LinkedList::empty();
        for i in v.iter() {
            list.append(*i);
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_list() -> LinkedList<i32> {
        let mut list = LinkedList::new(1);
        list.append(2);
        list.append(3);

        list
    }

    #[test]
    fn create() {
        let list: LinkedList<u8> = LinkedList::new(1);
        assert_eq!(list.head(), Some(&1));
        assert!(list.tail().is_none());
    }

    #[test]
    fn append() {
        let list = make_test_list();
        assert_eq!(list.len(), 3);
        assert_eq!(list.last(), Some(&3));
    }

    #[test]
    fn remove() {
        let mut list = make_test_list();
        list.append(4);
        list.append(5);

        list.remove_at(2);
        assert_eq!(list.len(), 4);
        assert_eq!(list.iter().collect::<Vec<&i32>>(), vec![&1, &2, &4, &5]);
    }

    #[test]
    fn last_equal_head() {
        let list = LinkedList::new(1);
        assert_eq!(list.last(), Some(&1i32));
    }

    #[test]
    fn nth() {
        let list = make_test_list();
        assert_eq!(list.nth(0), Some(&1));
    }

    #[test]
    fn iter_one() {
        let list = LinkedList::new(1);
        assert_eq!(list.iter().count(), 1);
    }

    #[test]
    fn iter_empty() {
        let list: LinkedList<u8> = LinkedList::empty();
        assert_eq!(list.iter().count(), 0);
    }

    #[test]
    fn iter_mut_empty() {
        let mut list: LinkedList<u8> = LinkedList::empty();
        assert_eq!(list.iter_mut().count(), 0);
    }

    #[test]
    fn iter() {
        let list = make_test_list();
        assert_eq!(list.iter().count(), 3);
    }

    #[test]
    fn iter_mut() {
        let mut list = make_test_list();
        assert_eq!(list.iter_mut().count(), 3);
    }

    #[test]
    fn from_iter() {
        let it = (0..5).into_iter();
        let list = LinkedList::from_iter(it);
        assert_eq!(list.len(), 5);
        assert_eq!(list.head(), Some(&0));
        assert_eq!(*list.iter().last().unwrap(), 4);
    }

    #[test]
    fn sort() {
        let list = make_test_list();
        let list = list.sort(Order::Descending);
        assert_eq!(list.iter().collect::<Vec<&i32>>(), vec![&3, &2, &1]);

        let list = list.sort(Order::Ascending);
        assert_eq!(list.iter().collect::<Vec<&i32>>(), vec![&1, &2, &3]);
    }

    #[test]
    fn index_trait() {
        let list = make_test_list();
        assert_eq!(list[1], 2);
    }
}
