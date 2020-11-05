//! A [BLAKE3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)-based
//! merkle (hash) tree implementation for superfast trees

use blake3::{self, Hash};
use std::rc::Rc;

/// A merkle tree
#[derive(Debug, Clone, PartialEq)]
pub struct Tree<T: AsRef<[u8]>> {
    /// Type of node contained inside the tree
    pub inner: NodeType<T>,
}

impl<T: AsRef<[u8]>> Tree<T> {
    /// Creates a new [Tree] based off of data supplied in `data`.
    pub fn new<D: Into<T>, DP: IntoIterator<Item = D>>(datapoints: DP) -> Self {
        let data_nodes = datapoints.into_iter().map(|data| Data::new(data));
        let mut bottom_nodes = vec![];

        let mut data_node_buf = None;

        for data_node in data_nodes {
            match data_node_buf {
                Some(_) => {
                    bottom_nodes.push(Node::from_data(data_node_buf.take().unwrap(), data_node))
                }
                None => data_node_buf = Some(data_node),
            }
        } // TODO: clone this and make it a recursive function for use upwards in middle nodes also

        unimplemented!()
    }

    /// Gets the hash for [Tree], stored in `inner.hash` typically
    pub fn hash(&self) -> Hash {
        match &self.inner {
            NodeType::Node(node) => node.hash,
            NodeType::Data(node) => node.hash,
        }
    }
}

/// Types of node that may be children
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType<T: AsRef<[u8]>> {
    Node(Node<T>),
    Data(Data<T>),
}

/// A middle-layer node, containing two nodes underneith that is of some [NodeType]
/// variation
#[derive(Debug, Clone, PartialEq)]
pub struct Node<T: AsRef<[u8]>> {
    pub hash: Hash,
    pub left: Rc<NodeType<T>>,
    pub right: Rc<NodeType<T>>,
}

impl<T: AsRef<[u8]>> Node<T> {
    /// Creates a new [Node] from given data for both left and right. Typically
    /// used internally for creating the bottom-most [Node] easily
    pub fn from_data(left: Data<T>, right: Data<T>) -> Self {
        let hash = blake3::hash(&[&left.hash.as_bytes()[..], &right.hash.as_bytes()[..]].concat());

        Self {
            hash,
            left: Rc::new(NodeType::Data(left)),
            right: Rc::new(NodeType::Data(right)),
        }
    }
}

/// The final datablock, containing the data needed
#[derive(Debug, Clone, PartialEq)]
pub struct Data<T: AsRef<[u8]>> {
    pub hash: Hash,
    pub data: T,
}

impl<T: AsRef<[u8]>> Data<T> {
    /// Creates a new [Data] from given `data`
    pub fn new<D: Into<T>>(data: D) -> Self {
        let data_into = data.into();

        Self {
            hash: blake3::hash(data_into.as_ref()),
            data: data_into.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_new_basic() {
        let bottom_left: Node<&str> = Node::from_data(Data::new("hello"), Data::new("there"));
        let bottom_right: Node<&str> = Node::from_data(Data::new("cool"), Data::new("person"));

        let hash = blake3::hash(
            &[
                &bottom_left.hash.as_bytes()[..],
                &bottom_right.hash.as_bytes()[..],
            ]
            .concat(),
        );

        let node = NodeType::Node(Node {
            hash,
            left: Rc::new(NodeType::Node(bottom_left)),
            right: Rc::new(NodeType::Node(bottom_right)),
        });

        assert_eq!(
            Tree::new(vec!["hello", "there", "cool", "person"]),
            Tree { inner: node }
        )
    }
}
