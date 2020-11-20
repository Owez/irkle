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
pub trait NodeMethod<T: AsRef<[u8]>> {
    /// Gets the [blake3]-based [Hash] for trait implementation, just call on any
    /// [Node], [Data] or [NodeType] like so: `item.get_hash()`. Typically all
    /// this method will do is get the `self.hash` but this can be used to adapt
    /// a broader [NodeType]
    fn get_hash(&self) -> Hash;

    /// Verifies the node down through recursion, providing a high-level
    /// checking/verification method
    ///
    /// If this fails, it will return the expected hash and the found hash where
    /// this hash failed at; this is formatted as `(expected_hash, found_node)`
    fn verify(&self) -> Result<(), (Hash, Hash)>;
}

/// A merkle tree
///
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
    /// Creates a new [Tree] based off of data supplied in `data`
    pub fn new<D: IntoIterator<Item = T>>(datapoints: D) -> Self {
        let mut data_nodes: Vec<Data<T>> = datapoints.into_iter().map(|d| Data::new(d)).collect();

        match data_nodes.len() {
            0 => panic!("Tree was given no datapoints and a merkle tree cannot be empty!"),
            1 => Self {
                inner: NodeType::Data(data_nodes.remove(0)),
            },
            _ => {
                /// Makes all levels of new nodes recursively from given
                /// originating [NodeType]s
                fn generate_nodes<T: AsRef<[u8]>, N: Into<NodeType<T>>>(
                    node_types: Vec<N>,
                ) -> NodeType<T> {
                    let mut output: Vec<NodeType<T>> = vec![];
                    let mut left_buf: Option<NodeType<T>> = None;

                    for node_type in node_types {
                        match left_buf {
                            Some(_) => output
                                .push(Node::new(left_buf.take().unwrap(), node_type.into()).into()),
                            None => left_buf = Some(node_type.into()),
                        }
                    }

                    output.extend(left_buf);

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
    }
}

impl<T: AsRef<[u8]>> NodeMethod<T> for Tree<T> {
    fn get_hash(&self) -> Hash {
        match &self.inner {
            NodeType::Node(node) => node.hash,
            NodeType::Data(node) => node.hash,
        }
    }

    fn verify(&self) -> Result<(), (Hash, Hash)> {
        self.inner.verify()
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

        Self {
            hash: hash_lr(&left_into, &right_into),
            left: Rc::new(left_into),
            right: Rc::new(right_into),
        }
    }
}

impl<T: AsRef<[u8]>> NodeMethod<T> for Node<T> {
    fn get_hash(&self) -> Hash {
        self.hash
    }

    fn verify(&self) -> Result<(), (Hash, Hash)> {
        self.left.verify()?;
        self.right.verify()?;

        let found_hash = hash_lr(&self.left, &self.right);

        if self.hash == found_hash {
            Ok(())
        } else {
            Err((found_hash, self.hash))
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

impl<T: AsRef<[u8]>> NodeMethod<T> for Data<T> {
    fn get_hash(&self) -> Hash {
        self.hash
    }

    fn verify(&self) -> Result<(), (Hash, Hash)> {
        let found_hash = blake3::hash(self.data.as_ref());

        if self.hash == found_hash {
            Ok(())
        } else {
            Err((found_hash, self.hash))
        }
    }
}

/// Types of node that may be children
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType<T: AsRef<[u8]>> {
    Node(Node<T>),
    Data(Data<T>),
}

impl<T: AsRef<[u8]>> NodeMethod<T> for NodeType<T> {
    fn get_hash(&self) -> Hash {
        match self {
            NodeType::Node(inner) => inner.hash,
            NodeType::Data(inner) => inner.hash,
        }
    }

    fn verify(&self) -> Result<(), (Hash, Hash)> {
        match self {
            NodeType::Node(inner) => inner.verify(),
            NodeType::Data(inner) => inner.verify(),
        }
    }
}

impl<T: AsRef<[u8]>> From<T> for NodeType<T> {
    /// Similar to the `impl<T: AsRef<[u8]>> From<Data<T>> for NodeType<T>` impl
    /// for [NodeType] but assumes raw input can also be a [Data]
    fn from(data: T) -> Self {
        NodeType::Data(Data::new(data))
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

/// Hashes left and right sides of a [NodeType], used for middle [Node]s
fn hash_lr<T: AsRef<[u8]>>(left: &NodeType<T>, right: &NodeType<T>) -> Hash {
    let mut hasher = blake3::Hasher::new();

    hasher.update(left.get_hash().as_bytes());
    hasher.update(right.get_hash().as_bytes());

    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Test data to use in testing
    const TEST_DATA: &str = "hello";

    /// Pseudo-random `u128` generator, used for minor fuzzing tests
    pub fn randish_128() -> u128 {
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();

        seed = seed << 13;
        seed = seed >> 4;

        seed << 5
    }

    #[test]
    fn hash_lr_check() {
        let data: Data<&str> = Data::new(TEST_DATA);
        let expected = blake3::hash(
            &[
                &data.get_hash().as_bytes()[..],
                &data.get_hash().as_bytes()[..],
            ]
            .concat(),
        );

        assert_eq!(
            hash_lr(&NodeType::from(data.clone()), &NodeType::from(data)),
            expected
        )
    }

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
        let bottom_left: Node<&str> = Node::new("hello", "there");
        let bottom_right: Node<&str> = Node::new("cool", "person");

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

    #[test]
    fn big_tree() {
        let mut data = vec![];

        for _ in 0..(randish_128() / 100000000000000000) {
            data.push(randish_128().to_be_bytes())
        }

        let tree: Tree<[u8; 16]> = Tree::new(data);

        assert_eq!(tree.verify(), Ok(()))
    }

    #[test]
    fn node_to_node_type() {
        let inner: Node<&str> = Node::new("", "").into();
        assert_eq!(NodeType::from(inner.clone()), NodeType::Node(inner))
    }

    #[test]
    fn data_to_node_type() {
        let inner: Data<&str> = Data::new("");
        assert_eq!(NodeType::from(inner.clone()), NodeType::Data(inner))
    }

    #[test]
    fn node_get_hash() {
        let node: Node<&str> = Node::new(TEST_DATA, TEST_DATA);

        assert_eq!(
            node.get_hash(),
            blake3::hash(
                &[
                    &blake3::hash(TEST_DATA.as_bytes()).as_bytes()[..],
                    &blake3::hash(TEST_DATA.as_bytes()).as_bytes()[..]
                ]
                .concat()
            )
        );
    }

    #[test]
    fn data_get_hash() {
        let data: Data<&str> = Data::new(TEST_DATA);
        assert_eq!(data.get_hash(), blake3::hash(TEST_DATA.as_bytes()));
    }

    #[test]
    #[should_panic]
    fn empty_tree() {
        let strings: Vec<String> = vec![];
        Tree::new(strings);
    }

    #[test]
    fn data_verification() {
        let mut test_struct: Data<&str> = Data::new(TEST_DATA);
        assert!(test_struct.verify().is_ok());

        test_struct.hash = blake3::hash(b"fknrejnfjrenf");
        assert!(test_struct.verify().is_err());
    }

    // TODO: more verification tests
}
