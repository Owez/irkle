//! Contains [Tree] retrieval implementation for child/parent [Node] getting

use super::Tree;
use crate::Node;

impl<T: AsRef<[u8]>> Tree<T> {
    /// Returns a reference to the parent of a given `ind` of a [Node] inside of the
    /// [Tree::inner] vector
    pub fn parent_of(&self, ind: usize) -> Option<&Node<T>> {
        if ind < 1 {
            None
        } else {
            self.inner.get((ind - 1) / 2)
        }
    }

    /// Returns a mutable reference to the parent of a given `ind` of a [Node] inside
    /// of the [Tree::inner] vector
    pub fn parent_of_mut(&mut self, ind: usize) -> Option<&mut Node<T>> {
        if ind < 1 {
            None
        } else {
            self.inner.get_mut((ind - 1) / 2)
        }
    }

    /// Returns a reference to the left child of a given `ind` of a [Node] inside of the
    /// [Tree::inner] vector
    pub fn left_of(&self, ind: usize) -> Option<&Node<T>> {
        match self.inner.get(ind) {
            Some(n) => {
                if n.data.is_some() {
                    return None;
                }
            }
            None => (),
        }

        self.inner.get(ind * 2 + 1)
    }

    /// Returns a mutable reference to the left child of a given `ind` of a [Node]
    /// inside of the [Tree::inner] vector
    pub fn left_of_mut(&self, ind: usize) -> Option<&Node<T>> {
        match self.inner.get(ind) {
            Some(n) => {
                if n.data.is_some() {
                    return None;
                }
            }
            None => (),
        }

        self.inner.get(ind * 2 + 1)
    }

    /// Returns a reference to the right child of a given `ind` of a [Node] inside of the
    /// [Tree::inner] vector
    pub fn right_of(&self, ind: usize) -> Option<&Node<T>> {
        match self.inner.get(ind) {
            Some(n) => {
                if n.data.is_some() {
                    return None;
                }
            }
            None => (),
        }

        self.inner.get(ind * 2 + 2)
    }

    /// Returns a mutable reference to the right child of a given `ind` of a [Node]
    /// inside of the [Tree::inner] vector
    pub fn right_of_mut(&self, ind: usize) -> Option<&Node<T>> {
        match self.inner.get(ind) {
            Some(n) => {
                if n.data.is_some() {
                    return None;
                }
            }
            None => (),
        }

        self.inner.get(ind * 2 + 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blake3;

    const TEST_DATA: &[u8] = b"hello world";

    #[test]
    fn parent_ref() {
        let hash = blake3::hash(TEST_DATA);

        let hash_node = Node::new_leaf(hash, hash);
        let data_node = Node::new_data(TEST_DATA);

        let tree = Tree {
            data: vec![],
            inner: vec![hash_node.clone(), data_node.clone(), data_node.clone()],
        };

        assert_eq!(tree.parent_of(0), None);
        assert_eq!(tree.parent_of(1), Some(&hash_node));
        assert_eq!(tree.parent_of(2), Some(&hash_node));
    }

    #[test]
    fn children_ref() {
        let hash = blake3::hash(TEST_DATA);

        let hash_node = Node::new_leaf(hash, hash);
        let data_node = Node::new_data(TEST_DATA);

        let tree = Tree {
            data: vec![],
            inner: vec![hash_node.clone(), data_node.clone(), data_node.clone()],
        };

        // left of get
        assert_eq!(tree.left_of(0), Some(&data_node));
        assert_eq!(tree.left_of(1), None);
        assert_eq!(tree.left_of(2), None);

        // right of get
        assert_eq!(tree.right_of(0), Some(&data_node));
        assert_eq!(tree.right_of(1), None);
        assert_eq!(tree.right_of(2), None);
    }
}
