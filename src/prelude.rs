pub trait Node {
    type Output;
    fn value(&self) -> &Self::Output;
}

