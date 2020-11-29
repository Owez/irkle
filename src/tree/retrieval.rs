//! Contains [Tree] retrieval implementation for child/parent [Node] getting

use super::Tree;
use crate::Node;

impl<T: AsRef<[u8]>> Tree<T> {
    /// Returns a reference to the parent of a given `ind` of a [Node] inside of the
    /// [Tree::inner] vector
    pub fn parent_of(&self, ind: usize) -> Option<&Node<T>> {
        self.inner.get(ind / 2)
    }

    /// Returns a mutable reference to the parent of a given `ind` of a [Node] inside
    /// of the [Tree::inner] vector
    pub fn parent_of_mut(&mut self, ind: usize) -> Option<&mut Node<T>> {
        self.inner.get_mut(ind / 2)
    }

    /// Returns a reference to the child of a given `ind` of a [Node] inside of the
    /// [Tree::inner] vector
    pub fn child_of(&self, ind: usize) -> Option<&Node<T>> {
        self.inner.get(ind * 2 + 1)
    }

    /// Returns a mutable reference to the child of a given `ind` of a [Node]
    /// inside of the [Tree::inner] vector
    pub fn child_of_mut(&self, ind: usize) -> Option<&Node<T>> {
        self.inner.get(ind * 2 + 1)
    }
}
