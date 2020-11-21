use crate::{Data, Node, NodeMethod, NodeType};
use blake3::Hash;

/// Makes all levels of new nodes recursively from given
/// originating [NodeType]s
fn generate_nodes<T: AsRef<[u8]>, N: Into<NodeType<T>>>(node_types: Vec<N>) -> NodeType<T> {
    let mut output: Vec<NodeType<T>> = vec![];
    let mut left_buf: Option<NodeType<T>> = None;

    for node_type in node_types {
        match left_buf {
            Some(_) => output.push(Node::new(left_buf.take().unwrap(), node_type.into()).into()),
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

/// A merkle tree -- *More documentation coming soon..*
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
            _ => Self {
                inner: generate_nodes(data_nodes),
            },
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Pseudo-random `u128` generator, used for minor fuzzing tests
    fn randish_128() -> u128 {
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();

        seed = seed << 13;
        seed = seed >> 4;

        seed << 5
    }

    #[test]
    fn new_two() {
        assert_eq!(
            Tree::new(vec!["left one", "right one"]),
            Tree {
                inner: NodeType::Node(Node::new(Data::new("left one"), Data::new("right one")))
            }
        )
    }

    #[test]
    fn new_odd() {
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
    fn new_four() {
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
    fn big() {
        let mut data = vec![];

        for _ in 0..(randish_128() / 100000000000000000) {
            data.push(randish_128().to_be_bytes())
        }

        let tree: Tree<[u8; 16]> = Tree::new(data);

        assert_eq!(tree.verify(), Ok(()))
    }

    #[test]
    #[should_panic]
    fn empty() {
        let strings: Vec<String> = vec![];
        Tree::new(strings);
    }
}
