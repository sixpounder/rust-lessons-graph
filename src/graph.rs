use std::{ops::Deref};

pub struct Graph<T = ()> {
    nodes: Vec<Node<T>>
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec![]
        }
    }

    pub fn spawn(&mut self, value: T) -> &Node<T> {
        let node = Node {
            value,
            edges: vec![],
            parent: self
        };

        let ln = self.nodes.len();
        self.nodes.push(node);
        self.nodes.get(ln).unwrap()
    }
}

pub struct Edge {
    weight: usize
}

pub struct Node<T> {
    value: T,
    edges: Vec<Edge>,
    parent: *const Graph<T>,
}

impl<T> Node<T> {
    pub fn get_parent(&self) -> &Graph<T> {
        unsafe {
            &*self.parent
        }
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
