//! Contains [Tree] creation implementations

use super::Tree;

impl<T: AsRef<[u8]>> Tree<T> {
    /// Creates a new [Tree] from given data
    pub fn new() -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Node;

    // #[test]
    // fn single_data() {
    //     let data_node = Node::from_data("hello there!");

    //     assert_eq!(Tree::new(vec![data_node.clone()]).inner, vec![data_node])
    // }
}
