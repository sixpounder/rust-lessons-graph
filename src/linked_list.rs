use std::{marker::PhantomData, ops::{Deref, DerefMut}, usize};

pub struct LinkedList<T> {
    value: T,
    next: Option<Box<LinkedList<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            size: 1,
        }
    }

    pub fn head(&self) -> &T {
        &self.value
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn tail(&self) -> Option<&LinkedList<T>> {
        let pointee = self.next.as_ref();
        match pointee {
            Some(ptr) => Some(ptr),
            None => None,
        }
    }

    pub fn tail_mut(&mut self) -> Option<&mut LinkedList<T>> {
        let pointee = self.next.as_mut();
        match pointee {
            Some(ptr) => Some(ptr),
            None => None,
        }
    }

    pub fn last(&self) -> &T {
        let mut current = self;
        while current.next.is_some() {
            current = current.next.as_ref().unwrap();
        }
        current
    }

    pub fn nth(&self, idx: usize) -> Option<&LinkedList<T>> {
        let mut current = self;
        let mut i = idx;
        while current.next.is_some() && i > 0 {
            current = current.next.as_ref().unwrap();
            i -= 1;
        }

        if i == 0 {
            // Not found
            None
        } else {
            Some(current)
        }
    }

    fn inner_last_mut(&mut self) -> &mut LinkedList<T> {
        let mut current = self;

        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        &mut *current
    }

    pub fn last_mut(&mut self) -> &mut T {
        self.inner_last_mut()
    }

    pub fn append(&mut self, value: T) {
        let mut last = self.inner_last_mut();
        let new_list = LinkedList::new(value);
        last.next = Some(Box::new(new_list));
        self.size += 1;
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator::new(self)
    }

    pub fn iter_mut(&mut self) -> LinkedListIteratorMut<T> {
        LinkedListIteratorMut::new(self)
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

// List iterators

pub struct LinkedListIterator<'a, T> {
    ptr: Option<&'a LinkedList<T>>,
}

impl<'a, T> LinkedListIterator<'a, T> {
    pub fn new(list: &'a LinkedList<T>) -> Self {
        Self { ptr: Some(list) }
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
        Self { ptr: list, marker: PhantomData }
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
        let list = LinkedList::new(1);
        assert_eq!(*list.head(), 1);
    }

    #[test]
    fn last_equal_head() {
        let list = LinkedList::new(1);
        assert_eq!(*list.last(), 1i32);
    }

    #[test]
    fn iter_one() {
        let list = LinkedList::new(1);
        assert_eq!(list.iter().count(), 1);
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
    fn append() {
        let list = make_test_list();
        assert_eq!(list.len(), 3);
        assert_eq!(*list.last(), 3);
    }
}
