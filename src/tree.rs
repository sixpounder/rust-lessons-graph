use std::ops::Deref;
use crate::prelude::Node;

pub enum DFTOrder {
    InOrder,
    PreOrder,
    PostOrder
}

pub trait NodeValue {}

impl<T: Sized> NodeValue for T {}

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
    pub fn iter_depth(&self) -> BTreeDFTIterator<T> {
        BTreeDFTIterator::new(self.get_root())
    }

    /// Creates a depth first traversal iterator on this tree with the specified traverse
    /// order algorythm
    pub fn iter_depth_order(&self, order: DFTOrder) -> BTreeDFTIterator<T> {
        BTreeDFTIterator::new_with_order(self.get_root(), order)
    }

    /// Creates a breadth first iterator on this tree
    pub fn iter_breadth(&self) -> BTreeDFTIterator<T> {
        todo!()
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

// Rust compiler complains of this because it MAY implement Iterator on Vec
// "sometime in the future" ¯\_(ツ)_/¯

// impl<G> From<Vec<G>> for BTree<G> {
//     fn from(vec: Vec<G>) -> Self {
//         if vec.len() == 0 {
//             BTree::empty()
//         } else {
//             let mut tree = BTree::new(vec[0]);
//             vec.iter().skip(1).for_each(|item| {
//                 tree.insert(item);
//             });

//             tree
//         }
//     }
// }

impl<T: PartialOrd + Clone> BTree<T> {
    pub fn insert(&mut self, value: T) {
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
                        deref.set_left_child(value.clone());
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
                        deref.set_right_child(value.clone());
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

/// Implements a Depth First Traversal for a given binary tree
pub struct BTreeDFTIterator<'a, T> {
    order: DFTOrder,
    root: Option<&'a BTreeNode<T>>,
    next: Option<&'a BTreeNode<T>>,
}

impl<'a, T> BTreeDFTIterator<'a, T> {
    pub fn new(root: Option<&'a BTreeNode<T>>) -> Self {
        Self {
            root,
            next: None,
            order: DFTOrder::InOrder
        }
    }

    pub fn new_with_order(root: Option<&'a BTreeNode<T>>, order: DFTOrder) -> Self {
        Self {
            root,
            next: None,
            order
        }
    }

    pub fn visit(&self, node: BTreeNode<T>) {}
}

impl<'a, T> Iterator for BTreeDFTIterator<'a, T> {
    type Item = BTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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
}
