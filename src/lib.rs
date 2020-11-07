//! A [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)-based
//! merkle (hash) tree implementation for superfast trees ⚡
//!
//! # Example
//!
//! ```rust
//! use irkle::Tree;
//!
//! fn main() {
//!     println!("{:#?}", Tree::new(vec!["hello", "there"]));
//! }
//! ```
//!
//! # Installation
//!
//! Simply add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [depedencies]
//! irkle = "0.1"
//! ```

use blake3::{self, Hash};
use std::rc::Rc;

/// Trait for running methods on any abstract kind of node, such as hash verification
/// or just getting the hash
pub trait NodeMethod {
    /// Gets the [blake3]-based [Hash] for trait implementation, just call on any
    /// [Node], [Data] or [NodeType] like so: `item.get_hash()`. Typically all
    /// this method will do is get the `self.hash` but this can be used to adapt
    /// a broader [NodeType]
    fn get_hash(&self) -> Hash;
}

/// A merkle tree

/// # Example
///
/// ```rs
/// use irkle::Tree;
///
/// fn main() {
///     println!("{:#?}", Tree::new(vec!["hello", "there"]));
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Tree<T: AsRef<[u8]>> {
    /// Type of node contained inside the tree or represents an empty tree
    pub inner: NodeType<T>,
}

impl<T: AsRef<[u8]>> Tree<T> {
    /// Creates a new [Tree] based off of data supplied in `data`.
    pub fn new<D: IntoIterator<Item = T>>(datapoints: D) -> Self {
        let mut data_nodes: Vec<Data<T>> = datapoints.into_iter().map(|d| Data::new(d)).collect();

        match data_nodes.len() {
            0 => panic!("Tree was given no datapoints and a merkle tree cannot be empty!"),
            1 => {
                return Self {
                    inner: NodeType::Data(data_nodes.remove(0)),
                }
            }
            _ => (),
        }

        /// Makes all levels of new nodes from given originating [NodeType]s
        fn generate_nodes<T: AsRef<[u8]>, N: Into<NodeType<T>>>(node_types: Vec<N>) -> NodeType<T> {
            let mut output: Vec<NodeType<T>> = vec![];
            let mut left_buf: Option<NodeType<T>> = None;

            for node_type in node_types {
                match left_buf {
                    Some(_) => {
                        output.push(Node::new(left_buf.take().unwrap(), node_type.into()).into())
                    }
                    None => left_buf = Some(node_type.into()),
                }
            }

            if left_buf.is_some() {
                output.push(left_buf.unwrap())
            }

            if output.len() == 1 {
                output.remove(0)
            } else {
                generate_nodes(output)
            }
        }

        Self {
            inner: generate_nodes(data_nodes),
        }
    }
}

impl<T: AsRef<[u8]>> NodeMethod for Tree<T> {
    fn get_hash(&self) -> Hash {
        match &self.inner {
            NodeType::Node(node) => node.hash,
            NodeType::Data(node) => node.hash,
        }
    }
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
    /// Creates a new [Node] from nodes below
    pub fn new<N: Into<NodeType<T>>>(left: N, right: N) -> Self {
        let left_into = left.into();
        let right_into = right.into();

        let hash = blake3::hash(
            &[
                &left_into.get_hash().as_bytes()[..],
                &right_into.get_hash().as_bytes()[..],
            ]
            .concat(),
        );

        Self {
            hash,
            left: Rc::new(left_into),
            right: Rc::new(right_into),
        }
    }
}

impl<T: AsRef<[u8]>> NodeMethod for Node<T> {
    fn get_hash(&self) -> Hash {
        self.hash
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

impl<T: AsRef<[u8]>> NodeMethod for Data<T> {
    fn get_hash(&self) -> Hash {
        self.hash
    }
}

/// Types of node that may be children
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType<T: AsRef<[u8]>> {
    Node(Node<T>),
    Data(Data<T>),
}

impl<T: AsRef<[u8]>> NodeMethod for NodeType<T> {
    fn get_hash(&self) -> Hash {
        match self {
            NodeType::Node(inner) => inner.hash,
            NodeType::Data(inner) => inner.hash,
        }
    }
}

impl<T: AsRef<[u8]>> From<Data<T>> for NodeType<T> {
    fn from(data: Data<T>) -> Self {
        NodeType::Data(data)
    }
}

impl<T: AsRef<[u8]>> From<Node<T>> for NodeType<T> {
    fn from(node: Node<T>) -> Self {
        NodeType::Node(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_new_two() {
        assert_eq!(
            Tree::new(vec!["left one", "right one"]),
            Tree {
                inner: NodeType::Node(Node::new(Data::new("left one"), Data::new("right one")))
            }
        )
    }

    #[test]
    fn tree_new_odd() {
        let left = NodeType::Node(Node::new(Data::new("this"), Data::new("is")));
        let right = NodeType::Data(Data::new("odd"));

        assert_eq!(
            Tree::new(vec!["this", "is", "odd"]),
            Tree {
                inner: NodeType::Node(Node::new(left, right))
            }
        )
    }

    #[test]
    fn tree_new_four() {
        let bottom_left: Node<&str> = Node::new(Data::new("hello"), Data::new("there"));
        let bottom_right: Node<&str> = Node::new(Data::new("cool"), Data::new("person"));

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
