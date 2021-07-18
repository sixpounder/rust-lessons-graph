use std::{iter::FromIterator, marker::PhantomData, ops::{Deref, DerefMut, Index}, usize};

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

/// An implementation of a linked list that supports empty sets.
pub struct LinkedList<T: Sized> {
    value: T,
    next: Option<Box<LinkedList<T>>>,
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
            size: 0
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
        return self.size == 0
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// The size of the list
    pub fn len(&self) -> usize {
        self.size
    }

    /// All of the list but its first item (its head)
    pub fn tail(&self) -> Option<&LinkedList<T>> {
        if self.is_empty() {
            None
        } else {
            match &self.next {
                Some(boxed_next) => {
                    Some(boxed_next.as_ref())
                },
                None => None
            }
        }
    }

    /// Same as `tail` but returns a mutable reference to the tail of the list
    pub fn tail_mut(&mut self) -> Option<&mut LinkedList<T>> {
        if self.is_empty() {
            None
        } else {
            match &mut self.next {
                Some(boxed_next) => {
                    Some(boxed_next.as_mut())
                },
                None => None
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
                    },
                    None => break
                }
            }

            Some(last)
        }
    }

    pub fn nth(&self, idx: usize) -> Option<&T> {
        self.iter().nth(idx)
    }

    pub fn last_mut(&mut self) -> Option<&mut LinkedList<T>> {
        if self.is_empty() {
            None
        } else {
            last_mut(self)
            // while last.has_next() {
            //     match last.next.as_mut() {
            //         Some(next) => {
            //             last = next.as_mut();
            //         },
            //         None => (),
            //     }
            // }
        
            // Some(last)
        }
    }

    pub fn append(&mut self, value: T) {
        if self.is_empty() {
            self.value = value;
        } else {
            let last = last_mut(self);
            let new_list = LinkedList::new(value);
            last.unwrap().next = Some(Box::new(new_list));
        }
        self.size += 1;
    }

    pub fn remove(&mut self, entry: &LinkedList<T>) {
        todo!()
    }

    // pub fn remove_index(&mut self, idx: usize) {
    //     if self.is_empty() || idx >= self.len() {
    //         return;
    //     }

    //     let mut i: usize = 0;
    //     let mut indirect: *mut LinkedList<T> = self;
    //     let mut prev = std::ptr::null_mut();
    //     while !indirect.is_null() && i < idx {
    //         unsafe {
    //             if let Some(next) = (*indirect).next.as_mut() {
    //                 prev = indirect;
    //                 indirect = &mut **next;
    //                 i += 1;
    //             }
    //         }
    //     }

    //     unsafe {
    //         // // Redefine pointers as mutable pointers
    //         // let indirect = indirect as *mut LinkedList<T>;
    //         // let prev = prev as *mut LinkedList<T>;


    //         // This point to the right side of the removable item
    //         let next_ptr = (*indirect).next.as_ref().unwrap().as_ptr();

    //         // This point to the left side
    //         let mut prev_next = (*prev).next.as_mut().unwrap().as_ptr_mut();

    //         // Link the two...
    //         *prev_next = *next_ptr;

    //         // ...and drop the removed one
    //         std::ptr::drop_in_place(indirect);

    //         self.size -= 1;
    //     }
    // }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator::new(self)
    }

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
    marker: PhantomData<&'a T>
}

impl<'a, T> LinkedListIteratorMut<'a, T> {
    pub fn new(list: &mut LinkedList<T>) -> Self {
        if list.is_empty() {
            Self { ptr: std::ptr::null_mut(), marker: PhantomData }
        } else {
            Self { ptr: list, marker: PhantomData }
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
                    },
                    None => {
                        self.ptr = std::ptr::null_mut();
                    }
                }
            }
        }

        if value.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *value)
            }
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

    // #[test]
    // fn remove() {
    //     let mut list = make_test_list();
    //     list.remove_index(1);
    //     assert_eq!(list.len(), 2);
    // }

    #[test]
    fn sort() {
        let list = make_test_list();
        let list = list.sort(Order::Descending);
        assert_eq!(list.iter().collect::<Vec<&i32>>(), vec![&3, &2, &1]);

        let list = list.sort(Order::Ascending);
        assert_eq!(list.iter().collect::<Vec<&i32>>(), vec![&1, &2, &3]);
    }
}
