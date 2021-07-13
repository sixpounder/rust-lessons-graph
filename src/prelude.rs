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
