//! A [BLAKE3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)-based
//! merkle (hash) tree implementation for superfast trees

use blake3::{self, Hash};
use std::rc::Rc;

/// Self-recursive function which makes many [Node]s, resulting in a final, single
/// [Node] containing all data below
fn make_middle_nodes<T: AsRef<[u8]>>(children_nodes: Vec<Node<T>>) -> Node<T> {
    let mut nodes = vec![];
    let mut node_buf = None;

    for data_node in children_nodes {
        match node_buf {
            Some(_) => nodes.push(Node::new(node_buf.take().unwrap(), data_node)),
            None => node_buf = Some(data_node),
        }
    }

    if nodes.len() == 1 {
        nodes.pop().unwrap()
    } else {
        make_middle_nodes(nodes)
    }
}

/// A merkle tree
#[derive(Debug, Clone, PartialEq)]
pub struct Tree<T: AsRef<[u8]>> {
    /// Type of node contained inside the tree
    pub inner: NodeType<T>,
}

impl<T: AsRef<[u8]>> Tree<T> {
    /// Creates a new [Tree] based off of data supplied in `data`.
    pub fn new<D: IntoIterator<Item = T>>(datapoints: D) -> Self {
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
        }

        if bottom_nodes.len() == 0 && data_node_buf.is_some() {
            return Self {
                inner: NodeType::Data(data_node_buf.take().unwrap()),
            };
        }

        // TODO: odd numbers

        Self {
            inner: NodeType::Node(make_middle_nodes(bottom_nodes)),
        }
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
    /// Creates a new [Node] from nodes below. Use [Node::from_data] for making
    /// new nodes containing data
    pub fn new(left: Node<T>, right: Node<T>) -> Self {
        let hash = blake3::hash(&[&left.hash.as_bytes()[..], &right.hash.as_bytes()[..]].concat());

        Self {
            hash,
            left: Rc::new(NodeType::Node(left)),
            right: Rc::new(NodeType::Node(right)),
        }
    }

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
        );
    }

    // TODO: test for odd number
}
