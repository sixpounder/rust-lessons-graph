use std::ops::Index;

pub struct Graph<'a, T> {
    nodes: Vec<Node<'a, T>>
}

impl<'a, T> Graph<'a, T> {
    pub fn spawn(&'a mut self, value: T) -> &Node<'a, T> {
        let new_node = Node {
            value,
            parent: self,
            out_connections: vec![]
        };

        let current_nodes_len = self.nodes.len();
        self.nodes.push(new_node);
        self.nodes.index(current_nodes_len)
    }
}

pub struct Node<'a, T> {
    value: T,
    parent: &'a Graph<'a, T>,
    out_connections: Vec<&'a Node<'a, T>>
}

impl<'a, T> Node<'a, T> {
    pub fn connect_to(&mut self, target: &'a Node<'_, T>) {
        self.out_connections.push(target)
    }

    pub fn get_outgoing_connections(&self) -> &Vec<&Node<T>> {
        &self.out_connections
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn incoming_connections_iter(&'a self) -> IncomingConnectionIterator<'a, T> {
        IncomingConnectionIterator {
            node: &self
        }
    }
}

pub struct IncomingConnectionIterator<'a, T> {
    node: &'a Node<'a, T>
}

impl<T> IncomingConnectionIterator<'_, T> {
    pub fn parent_graph(&self) -> &Graph<T> {
        self.node.parent
    }
}

impl<'a, T> Iterator for IncomingConnectionIterator<'a, T> {
    type Item = &'a Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// // for incoming in some_node.incoming_connections_iter() { ... }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// pub struct IterableString {
//     value: String
// }

// impl IterableString {
//     pub fn iter(&self) -> StringIterator {
//         StringIterator {
//             originalValue: self,
//             currentPosition: 0
//         }
//     }

//     pub fn get_string(&self) -> String {
//         self.value.clone()
//     }
// }

// pub struct StringIterator<'a> {
//     originalValue: &'a IterableString,
//     currentPosition: usize
// }

// impl Iterator for StringIterator<'_> {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         let the_string = self.originalValue.get_string();
//         if self.currentPosition < self.originalValue.get_string().len() {
//             let next_char = the_string.index(self.currentPosition..(self.currentPosition + 1));
//             Some(String::from(next_char))
//         } else {
//             None
//         }
//     }
// }

// pub struct Fibonacci {
//     previous: u64,
//     current: u64,
//     count: Option<usize>
// }

// impl Iterator for Fibonacci {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.count {
//             Some(c) => {
//                 if c == 0 {
//                     None
//                 } else {
//                     let next_number = self.previous + self.current;
//                     self.previous = self.current;
//                     self.current = next_number;
//                     self.count = Some(c - 1);
//                     Some(next_number)
//                 }
//             },
//             None => {
//                 let next_number = self.previous + self.current;
//                 self.previous = self.current;
//                 self.current = next_number;
//                 Some(next_number)
//             }
//         }
//     }
// }

// for n in fib { println... }