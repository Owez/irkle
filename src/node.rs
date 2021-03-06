use crate::{NodeMethod, NodeType};
use blake3::Hash;

/// Hashes left and right sides of a [NodeType], used for middle [Node]s
fn hash_lr<T: AsRef<[u8]>>(left: &NodeType<T>, right: &NodeType<T>) -> Hash {
    let mut hasher = blake3::Hasher::new();

    hasher.update(left.get_hash().as_bytes());
    hasher.update(right.get_hash().as_bytes());

    hasher.finalize()
}

/// A middle-layer node, containing two nodes underneith that is of a [NodeType]
/// variation
///
/// # Verification
///
/// This structure implements the [NodeMethod] trait and can therefore be
/// verified using the `.verify()` method. If ran on this particular structure,
/// it will recurse into both the `left` and `right` [NodeType]'s and check them
/// for any inconsistancies.
///
/// You may want to check [Tree](crate::Tree)'s verification if you would like
/// to verify a whole tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<T: AsRef<[u8]>> {
    pub hash: Hash,
    pub left: Box<NodeType<T>>,
    pub right: Box<NodeType<T>>,
}

impl<T: AsRef<[u8]>> Node<T> {
    /// Creates a new [Node] from any [NodeType]s below
    pub fn new<N: Into<NodeType<T>>>(left: N, right: N) -> Self {
        let left_into = left.into();
        let right_into = right.into();

        Self {
            hash: hash_lr(&left_into, &right_into),
            left: Box::new(left_into),
            right: Box::new(right_into),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Data;

    const TEST_DATA: &str = "hello";

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
    fn node_to_node_type() {
        let inner: Node<&str> = Node::new("", "").into();
        assert_eq!(NodeType::from(inner.clone()), NodeType::Node(inner))
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
    fn verify_ok() {
        let data: Data<&str> = Data::new(TEST_DATA);
        let node = Node::new(data.clone(), data);
        assert!(node.verify().is_ok());
    }

    #[test]
    fn verify_bad() {
        let data: Data<&str> = Data::new(TEST_DATA);
        let mut node = Node::new(data.clone(), data);

        let bad = blake3::hash(b"bad");

        match node.left.as_mut() {
            NodeType::Data(d) => d.hash = bad,
            _ => panic!(),
        }

        assert_eq!(
            node.verify(),
            Err((blake3::hash(TEST_DATA.as_bytes()), bad))
        );
    }
}
