//! Contains [Node] creation implementations

use super::{Node, NodeInner};
use blake3;

/// Hashes left and right sides of a [NodeType], used for middle [Node]s
fn hash_lr(left: blake3::Hash, right: blake3::Hash) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();

    hasher.update(left.as_bytes());
    hasher.update(right.as_bytes());

    hasher.finalize()
}

impl<T: AsRef<[u8]>> Node<T> {
    /// Creates a new leaf-style [Node] from a present `left` and `right` hashes
    pub fn new_leaf(left: blake3::Hash, right: blake3::Hash) -> Self {
        Node {
            hash: hash_lr(left, right),
            inner: NodeInner::Leaf,
        }
    }

    /// Creates a new datablock-style [Node] from given raw data
    pub fn new_data(data: T) -> Self {
        Node {
            hash: blake3::hash(data.as_ref()),
            inner: NodeInner::Data(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &[u8] = b"Hello world";

    #[test]
    fn simple_leaf() {
        let hash = blake3::hash(TEST_DATA);
        let node_leaf: Node<&str> = Node::new_leaf(hash, hash);

        assert_eq!(
            node_leaf,
            Node {
                hash: hash_lr(hash, hash),
                inner: NodeInner::Leaf
            }
        )
    }

    #[test]
    fn simple_data() {
        assert_eq!(
            Node::new_data(TEST_DATA),
            Node {
                hash: blake3::hash(TEST_DATA),
                inner: NodeInner::Data(TEST_DATA)
            }
        )
    }
}
