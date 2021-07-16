#[macro_export]
macro_rules! list {
    () => {
        let mut list = LinkedList::empty();
    };
    ( $item:expr ) => {
        list!($item)
    };
    ( $head:expr, $( $tail:expr ),* ) => {
        {
            let mut list = LinkedList::new($head);
            $(
                list.append($tail);
            )*
            list
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::LinkedList;
    #[test]
    fn macro_create() {
        let list = list! (1, 2, 3);
        assert_eq!(list.len(), 3);
    }
}
