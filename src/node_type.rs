use crate::{Data, Node, NodeMethod};
use blake3::Hash;

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
