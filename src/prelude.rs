///# This module contains common traits and types usefull to work with the crate

#[derive(Debug, PartialEq)]
pub enum Order {
    Ascending,
    Descending
}
pub enum DFTOrder {
    InOrder,
    PreOrder,
    PostOrder
}

pub trait Node {
    type Output;
    fn value(&self) -> &Self::Output;
}

pub trait Traversable<T> {
    fn traverse<F>(&self, order: DFTOrder, f: &F) where F: Fn(&T);
}

pub trait Sortable {
    fn sort(self, order: Order) -> Self;
}
