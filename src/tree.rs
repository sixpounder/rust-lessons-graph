use crate::prelude::{DFTOrder, Node, Traversable};
use std::ops::Deref;

pub trait NodeValue {}

impl<T: Sized> NodeValue for T {}

/// Implements a binary tree
pub struct BTree<T> {
    root: Option<BTreeNode<T>>,
}

impl<T> BTree<T> {
    pub fn empty() -> Self {
        Self { root: None }
    }

    pub fn new(root: T) -> Self {
        Self {
            root: Some(BTreeNode::new(root)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn get_root(&self) -> Option<&BTreeNode<T>> {
        self.root.as_ref()
    }

    /// Creates a depth first traversal iterator on this tree, with an *in odrder*
    /// traversal algorythm
    pub fn iter_depth(&self) -> BTreeInOrderIterator<T> {
        BTreeInOrderIterator::new(self.get_root())
    }

    /// Creates a depth first traversal iterator on this tree with the specified traversing
    /// order algorithm
    pub fn iter_depth_order<'a>(
        &'a self,
        order: DFTOrder,
    ) -> Box<dyn Iterator<Item = &BTreeNode<T>> + 'a> {
        match order {
            DFTOrder::InOrder => Box::new(BTreeInOrderIterator::new(self.get_root())),
            DFTOrder::PreOrder => Box::new(BTreePreOrderIterator::new(self.get_root())),
            DFTOrder::PostOrder => Box::new(BTreePostOrderIterator::new(self.get_root())),
        }
    }

    /// Creates a breadth first iterator on this tree
    pub fn iter_breadth(&self) {
        todo!()
    }
}

impl<T> Traversable<T> for BTree<T> {
    fn traverse<F>(&self, order: DFTOrder, f: &F)
    where
        F: Fn(&T),
    {
        match self.get_root() {
            Some(root) => {
                match order {
                    DFTOrder::InOrder => root.visit_in_order(f),
                    DFTOrder::PreOrder => root.visit_pre_order(f),
                    DFTOrder::PostOrder => root.visit_post_order(f),
                };
            }
            None => (),
        }
    }
}

impl<G: Iterator> From<G> for BTree<<G as Iterator>::Item>
where
    <G as Iterator>::Item: PartialOrd + Clone,
{
    fn from(mut iterable: G) -> Self {
        match iterable.next() {
            Some(first_value) => {
                let mut tree = BTree::new(first_value);

                iterable.for_each(|item| {
                    tree.insert(item);
                });

                tree
            }
            None => BTree::empty(),
        }
    }
}

impl<T: PartialOrd> BTree<T> {
    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root.as_mut().unwrap().value = value;
            return;
        }

        let mut current: &mut BTreeNode<T> = &mut self.root.as_mut().unwrap();
        let mut parent: *mut BTreeNode<T>;

        loop {
            parent = current.as_ptr_mut();
            let deref;
            unsafe {
                deref = parent.as_mut().unwrap();
            }
            if value < *deref.value() {
                // Go to the left of the tree
                let left_node = current.get_left_child_mut();
                match left_node {
                    Some(left_node) => {
                        current = left_node;
                        continue;
                    }
                    None => {
                        // Insert to the left
                        deref.set_left_child(value);
                        break;
                    }
                }
            } else {
                // Go to the right of the tree
                let right_node = current.get_right_child_mut();
                match right_node {
                    Some(right_node) => {
                        current = right_node;
                        continue;
                    }
                    None => {
                        deref.set_right_child(value);
                        break;
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, _node: &BTreeNode<T>) {
        todo!()
    }
}

pub struct BTreeInOrderIterator<'a, T> {
    next: Option<&'a BTreeNode<T>>,
    stack: Vec<&'a BTreeNode<T>>,
}

impl<'a, T> BTreeInOrderIterator<'a, T> {
    pub fn new(root: Option<&'a BTreeNode<T>>) -> Self {
        match root {
            Some(node) => Self {
                next: Some(node),
                stack: vec![],
            },
            None => Self {
                next: None,
                stack: vec![],
            },
        }
    }
}

impl<'a, T> Iterator for BTreeInOrderIterator<'a, T> {
    type Item = &'a BTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next.is_some() {
            let unwrapped_next = self.next.unwrap();
            self.stack.push(unwrapped_next);
            self.next = unwrapped_next.get_left_child();
        }

        if self.stack.is_empty() {
            return self.next;
        } else {
            let last_current = self.stack.pop().unwrap(); // unwrap because stack is certainly NOT empty
            self.next = last_current.get_right_child();

            Some(last_current)
        }
    }
}

pub struct BTreePreOrderIterator<'a, T> {
    stack: Vec<&'a BTreeNode<T>>,
}

impl<'a, T> BTreePreOrderIterator<'a, T> {
    pub fn new(root: Option<&'a BTreeNode<T>>) -> Self {
        match root {
            Some(node) => Self { stack: vec![node] },
            None => Self { stack: vec![] },
        }
    }
}

impl<'a, T> Iterator for BTreePreOrderIterator<'a, T> {
    type Item = &'a BTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.stack.pop();
        match next {
            Some(popped_node) => {
                if let Some(right) = popped_node.get_right_child() {
                    self.stack.push(right);
                }

                if let Some(left) = popped_node.get_left_child() {
                    self.stack.push(left);
                }
            }
            None => (),
        }

        next
    }
}

pub struct BTreePostOrderIterator<'a, T> {
    root: Option<&'a BTreeNode<T>>,
    next: Option<&'a BTreeNode<T>>,
    stack1: Vec<&'a BTreeNode<T>>,
    stack2: Vec<&'a BTreeNode<T>>,
}

impl<'a, T> BTreePostOrderIterator<'a, T> {
    pub fn new(root: Option<&'a BTreeNode<T>>) -> Self {
        Self {
            root,
            next: None,
            stack1: vec![root.unwrap()],
            stack2: vec![],
        }
    }
}

impl<'a, T> Iterator for BTreePostOrderIterator<'a, T> {
    type Item = &'a BTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.root.is_none() {
            None
        } else {
            self.next = self.stack1.pop();
            self.stack2.push(self.next.unwrap());

            match self.next.unwrap().get_left_child() {
                Some(left_node) => {
                    self.stack1.push(left_node);
                },
                None => (),
            }
            match self.next.unwrap().get_right_child() {
                Some(right_node) => {
                    self.stack1.push(right_node);
                },
                None => (),
            }

            self.next
        }
    }
}

pub struct BTreeNode<T: NodeValue> {
    value: T,
    left: Option<Box<BTreeNode<T>>>,
    right: Option<Box<BTreeNode<T>>>,
}

impl<T> Node for BTreeNode<T> {
    type Output = T;
    fn value(&self) -> &T {
        &self.value
    }
}

impl<T> PartialEq for BTreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl<T: NodeValue> Deref for BTreeNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value()
    }
}

impl<T: NodeValue> BTreeNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn as_ptr(&self) -> *const BTreeNode<T> {
        self as *const BTreeNode<T>
    }

    pub fn as_ptr_mut(&mut self) -> *mut BTreeNode<T> {
        self as *mut BTreeNode<T>
    }

    fn set_left_child(&mut self, value: T) {
        self.left = Some(Box::new(BTreeNode::new(value)))
    }

    fn set_right_child(&mut self, value: T) {
        self.right = Some(Box::new(BTreeNode::new(value)))
    }

    pub fn get_left_child(&self) -> Option<&BTreeNode<T>> {
        match self.left.as_ref() {
            Some(v) => Some(v),
            None => None,
        }
    }

    fn get_left_child_mut(&mut self) -> Option<&mut Box<BTreeNode<T>>> {
        self.left.as_mut()
    }

    pub fn get_right_child(&self) -> Option<&BTreeNode<T>> {
        match self.right.as_ref() {
            Some(v) => Some(v),
            None => None,
        }
    }

    fn get_right_child_mut(&mut self) -> Option<&mut Box<BTreeNode<T>>> {
        self.right.as_mut()
    }

    fn visit_in_order<F>(&self, f: &F)
    where
        F: Fn(&T),
    {
        match self.get_left_child() {
            Some(left_child) => left_child.visit_in_order(f),
            None => (),
        }

        f(self);

        match self.get_right_child() {
            Some(right_child) => right_child.visit_in_order(f),
            None => (),
        }
    }

    fn visit_pre_order<F>(&self, f: &F)
    where
        F: Fn(&T),
    {
        f(self);

        match self.get_left_child() {
            Some(left_child) => left_child.visit_pre_order(f),
            None => (),
        }

        match self.get_right_child() {
            Some(right_child) => right_child.visit_pre_order(f),
            None => (),
        }
    }

    fn visit_post_order<F>(&self, f: &F)
    where
        F: Fn(&T),
    {
        match self.get_left_child() {
            Some(left_child) => left_child.visit_post_order(f),
            None => (),
        }

        match self.get_right_child() {
            Some(right_child) => right_child.visit_post_order(f),
            None => (),
        }

        f(self);
    }
}

impl<T> Traversable<T> for BTreeNode<T> {
    fn traverse<F>(&self, order: DFTOrder, f: &F)
    where
        F: Fn(&T),
    {
        match order {
            DFTOrder::InOrder => self.visit_in_order(f),
            DFTOrder::PreOrder => self.visit_pre_order(f),
            DFTOrder::PostOrder => self.visit_post_order(f),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut tree = BTree::new(10u8);
        tree.insert(2);
        tree.insert(11);
        tree.insert(3);
        let new_node_left = tree.get_root().unwrap().get_left_child().unwrap();
        let new_node_right = tree.get_root().unwrap().get_right_child().unwrap();
        let new_node_three = tree
            .get_root()
            .unwrap()
            .get_left_child()
            .unwrap()
            .get_right_child()
            .unwrap();
        assert_eq!(**new_node_left, 2u8);
        assert_eq!(**new_node_right, 11u8);
        assert_eq!(**new_node_three, 3u8);
    }

    #[test]
    fn from_iterable() {
        let vec: Vec<u8> = vec![12, 3, 5];
        let tree: BTree<&u8> = BTree::from(vec.iter());
        assert_eq!(***tree.get_root().unwrap(), 12);
        assert_eq!(***tree.get_root().unwrap().get_left_child().unwrap(), 3);
    }

    #[test]
    fn in_order_iter() {
        let vec: Vec<u8> = vec![12, 15, 3, 5];
        let tree: BTree<&u8> = BTree::from(vec.iter());
        let out_vec = tree.iter_depth().collect::<Vec<&BTreeNode<&u8>>>();
        assert_eq!(out_vec.len(), 4);
        assert_eq!(**out_vec[0], &3u8);
        assert_eq!(**out_vec[1], &5u8);
        assert_eq!(**out_vec[2], &12u8);
        assert_eq!(**out_vec[3], &15u8);
    }

    #[test]
    fn pre_order_iter() {
        let vec: Vec<u8> = vec![12, 15, 3, 5];
        let tree: BTree<&u8> = BTree::from(vec.iter());
        let out_vec = tree.iter_depth_order(DFTOrder::PreOrder).collect::<Vec<&BTreeNode<&u8>>>();
        assert_eq!(out_vec.len(), 4);
        assert_eq!(**out_vec[0], &12u8);
        assert_eq!(**out_vec[1], &3u8);
        assert_eq!(**out_vec[2], &5u8);
        assert_eq!(**out_vec[3], &15u8);
    }

    #[test]
    fn post_order_iter() {
        let vec: Vec<u8> = vec![12, 15, 3, 5];
        let tree: BTree<&u8> = BTree::from(vec.iter());
        let out_vec = tree.iter_depth_order(DFTOrder::PostOrder).collect::<Vec<&BTreeNode<&u8>>>();
        assert_eq!(out_vec.len(), 4);
        assert_eq!(**out_vec[0], &12u8);
        assert_eq!(**out_vec[1], &3u8);
        assert_eq!(**out_vec[2], &5u8);
        assert_eq!(**out_vec[3], &15u8);
    }
}
