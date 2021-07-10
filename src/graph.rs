use std::{ops::Deref};

use crate::prelude::Node;

pub struct Graph<T = ()> {
    nodes: Vec<GraphNode<T>>
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec![]
        }
    }

    pub fn spawn(&mut self, value: T) -> &GraphNode<T> {
        let node = GraphNode {
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

pub struct GraphNode<T> {
    value: T,
    edges: Vec<Edge>,
    parent: *const Graph<T>,
}

impl<T> Node for GraphNode<T> {
    type Output = T;

    fn value(&self) -> &Self::Output {
        &self.value
    }
}

impl<T> Deref for GraphNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> GraphNode<T> {
    pub fn get_parent(&self) -> &Graph<T> {
        unsafe {
            &*self.parent
        }
    }
}
