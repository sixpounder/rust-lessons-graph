use std::ops::Deref;

pub trait NodeValue {}

impl<T: Sized> NodeValue for T {}

pub struct BTree<T> {
    root: Option<Node<T>>,
}

impl<T> BTree<T> {
    pub fn empty() -> Self {
        Self { root: None }
    }

    pub fn new(root: T) -> Self {
        Self {
            root: Some(Node::new(root)),
        }
    }

    pub fn get_root(&self) -> Option<&Node<T>> {
        self.root.as_ref()
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

                iterable.skip(1).for_each(|item| {
                    tree.insert(item);
                });

                tree
            }
            None => BTree::empty(),
        }
    }
}

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
        let mut current: &mut Node<T> = &mut self.root.as_mut().unwrap();
        let mut parent: *mut Node<T>;

        loop {
            parent = current.as_ptr_mut();
            let deref;
            unsafe {
                deref = parent.as_mut().unwrap();
            }
            if value < *deref.get_value() {
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

    pub fn remove(&mut self, _node: &Node<T>) {
        todo!()
    }
}

// pub enum TreeIterMode {
//     DepthFirst,
//     BreadthFirst,
// }

// pub struct TreeDFTIterator<'a, T> {
//     tree: &'a BTree<T>,
//     root: &'a Node<T>,
//     next: Option<&'a Node<T>>,
// }

// impl<'a, T> TreeDFTIterator<'a, T> {
//     pub fn visit(&self, node: Node<T>) {}
// }

// impl<'a, T> Iterator for TreeDFTIterator<'a, T> {
//     type Item = Node<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

pub struct Node<T: NodeValue> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl<T: NodeValue> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: NodeValue> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn as_ptr(&self) -> *const Node<T> {
        self as *const Node<T>
    }

    pub fn as_ptr_mut(&mut self) -> *mut Node<T> {
        self as *mut Node<T>
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    fn set_left_child(&mut self, value: T) {
        self.left = Some(Box::new(Node::new(value)))
    }

    fn set_right_child(&mut self, value: T) {
        self.right = Some(Box::new(Node::new(value)))
    }

    pub fn get_left_child(&self) -> Option<&Node<T>> {
        match self.left.as_ref() {
            Some(v) => Some(v),
            None => None,
        }
    }

    fn get_left_child_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        self.left.as_mut()
    }

    pub fn get_right_child(&self) -> Option<&Node<T>> {
        match self.right.as_ref() {
            Some(v) => Some(v),
            None => None,
        }
    }

    fn get_right_child_mut(&mut self) -> Option<&mut Box<Node<T>>> {
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
        assert_eq!(***tree.get_root().unwrap().get_left_child().unwrap(), 5);
    }
}
