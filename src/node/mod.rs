//! Contains [Node] and related implementations, see item-level documentation for
//! more information

use blake3;

/// A single node on the [Irkle] tree which maps to a [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)-based
/// hash and optionally some data
///
/// If this does contain some data, this would now be considered a "data block".
/// If this is not the case, this node would simply be referred to as a leaf in
/// the tree.
pub struct Node<T: AsRef<[u8]>> {
    /// Required hash of this node
    pub hash: blake3::Hash,

    /// Optional data contained inside of this node
    ///
    /// If this infomation is present (e.g. an [Option::Some] value), this node
    /// is considered a "data block" and if not, it is assumed to be a leaf.
    pub data: Option<T>,
}

/// Tests for [Node]-related operations
#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test
}
