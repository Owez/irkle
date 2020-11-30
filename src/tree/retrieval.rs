//! Contains [Tree] retrieval implementation for child/parent [Node] getting

use super::Tree;
use crate::{Node, NodeInner};

impl<T: AsRef<[u8]>> Tree<T> {
    /// Returns a reference to the parent of a given `ind` of a [Node] inside of the
    /// [Tree::inner] vector
    pub fn parent(&self, ind: usize) -> Option<&Node<T>> {
        if ind < 1 {
            None
        } else {
            self.inner.get((ind - 1) / 2)
        }
    }

    /// Returns a mutable reference to the parent of a given `ind` of a [Node] inside
    /// of the [Tree::inner] vector
    pub fn parent_mut(&mut self, ind: usize) -> Option<&mut Node<T>> {
        if ind < 1 {
            None
        } else {
            self.inner.get_mut((ind - 1) / 2)
        }
    }

    /// Returns a reference to the left child of a given `ind` of a [Node] inside of the
    /// [Tree::inner] vector
    pub fn left(&self, ind: usize) -> Option<&Node<T>> {
        match self.inner.get(ind) {
            Some(n) => match n.inner {
                NodeInner::Data(_) => return None,
                NodeInner::Padding => return None,
                _ => (),
            },
            None => (),
        }

        self.inner.get(ind * 2 + 1)
    }

    /// Returns a mutable reference to the left child of a given `ind` of a [Node]
    /// inside of the [Tree::inner] vector
    pub fn left_mut(&mut self, ind: usize) -> Option<&mut Node<T>> {
        match self.inner.get(ind) {
            Some(n) => match n.inner {
                NodeInner::Data(_) => return None,
                NodeInner::Padding => return None,
                _ => (),
            },
            None => (),
        }

        self.inner.get_mut(ind * 2 + 1)
    }

    /// Returns a reference to the right child of a given `ind` of a [Node] inside of the
    /// [Tree::inner] vector
    pub fn right(&self, ind: usize) -> Option<&Node<T>> {
        match self.inner.get(ind) {
            Some(n) => match n.inner {
                NodeInner::Data(_) => return None,
                NodeInner::Padding => return None,
                _ => (),
            },
            None => (),
        }

        self.inner.get(ind * 2 + 2)
    }

    /// Returns a mutable reference to the right child of a given `ind` of a [Node]
    /// inside of the [Tree::inner] vector
    pub fn right_mut(&mut self, ind: usize) -> Option<&mut Node<T>> {
        match self.inner.get(ind) {
            Some(n) => match n.inner {
                NodeInner::Data(_) => return None,
                NodeInner::Padding => return None,
                _ => (),
            },
            None => (),
        }

        self.inner.get_mut(ind * 2 + 2)
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

        assert_eq!(tree.parent(0), None);
        assert_eq!(tree.parent(1), Some(&hash_node));
        assert_eq!(tree.parent(2), Some(&hash_node));
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
        assert_eq!(tree.left(0), Some(&data_node));
        assert_eq!(tree.left(1), None);
        assert_eq!(tree.left(2), None);

        // right of get
        assert_eq!(tree.right(0), Some(&data_node));
        assert_eq!(tree.right(1), None);
        assert_eq!(tree.right(2), None);
    }

    #[test]
    fn parent_mut() {
        let hash = blake3::hash(TEST_DATA);

        let mut hash_node = Node::new_leaf(hash, hash);
        let data_node = Node::new_data(TEST_DATA);

        let mut tree = Tree {
            data: vec![],
            inner: vec![hash_node.clone(), data_node.clone(), data_node.clone()],
        };

        assert_eq!(tree.parent_mut(0), None);
        assert_eq!(tree.parent_mut(1), Some(&mut hash_node));
        assert_eq!(tree.parent_mut(2), Some(&mut hash_node));
    }

    #[test]
    fn children_mut() {
        let hash = blake3::hash(TEST_DATA);

        let hash_node = Node::new_leaf(hash, hash);
        let mut data_node = Node::new_data(TEST_DATA);

        let mut tree = Tree {
            data: vec![],
            inner: vec![hash_node.clone(), data_node.clone(), data_node.clone()],
        };

        // left of get
        assert_eq!(tree.left_mut(0), Some(&mut data_node));
        assert_eq!(tree.left_mut(1), None);
        assert_eq!(tree.left_mut(2), None);

        // right of get
        assert_eq!(tree.right_mut(0), Some(&mut data_node));
        assert_eq!(tree.right_mut(1), None);
        assert_eq!(tree.right_mut(2), None);
    }
}
