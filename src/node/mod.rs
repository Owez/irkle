//! Contains [Node] and related implementations, see item-level documentation for
//! more information

mod new;

pub use new::*;

use blake3;

/// A single node on the main irkle [Tree](crate::Tree) which maps to a
/// [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)-based
/// hash and optionally some data
///
/// If this does contain some data, this would now be considered a "data block".
/// If this is not the case, this node would simply be referred to as a leaf in
/// the tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T: AsRef<[u8]>> {
    /// Required hash of this node
    pub hash: blake3::Hash,

    /// Enum which determines the type of the node and the data, if any, contained
    /// inside; see [NodeInner] for more information
    pub inner: NodeInner<T>,
}

/// Determines the type of a [Node], typically a leaf node which contains its
/// children hashes or a data block node which contains a data and it's hash
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeInner<T: AsRef<[u8]>> {
    /// A leaf node which represents the children hashes, e.g. `H3 = H1 + H2`
    Leaf,

    /// Padding if the length of all the [Node]s with [NodeInner::Data] are of an
    /// odd number
    Padding,

    /// A data block, contained at the end of a [Tree](crate::Tree) which may be
    /// padded with a [Node] of [NodeInner::Padding] if odd in length overall
    Data(T),
}
