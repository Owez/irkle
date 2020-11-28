//! A merkle tree implementation based upon the [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)
//! hash and [binary tree arrays](https://en.wikipedia.org/wiki/Binary_tree#Arrays)
//! for superfast trees ⚡
//!
//! Please travel to the primary [Irkle] structure of this library for more
//! documentation then this module-level infomation can provide, as it is the main
//! implemented utility that this library aims to provide.

use blake3;
use std::rc::Weak;

/// The primary tree used inside of irkle, uses an [array-based](https://en.wikipedia.org/wiki/Binary_tree#Arrays)
/// method of storage
///
/// # Tree structure
///
/// Given a tree looking similar to the following:
///
/// ```none
/// alpha
///     bravo
///         charlie
///         delta
///     echo
///         foxtrot
///         golf
/// ```
///
/// It would be translated into a list similar to the following:
///
/// ```none
/// [alpha, bravo, charlie, delta, echo, foxtrot, golf]
/// ```
///
/// This allows the tree to have better [cache locality](https://en.wikipedia.org/wiki/Locality_of_reference)
/// inside of the CPU's [cache](https://en.wikipedia.org/wiki/CPU_cache) and the
/// struture of the tree is implied by the position of each element inside of
/// this list.
///
/// Unique to this merkle (hash) tree implementation, we also include a list of
/// references to each bottom-most [data block](https://en.wikipedia.org/wiki/Block_(data_storage))
/// for improved preformance when searching, which would look something like the
/// following:
///
/// ```none
/// [&charlie, &delta, &foxtrot, &golf]
/// ```
///
/// This data structure may be put to use without needing to be transformed into
/// something else in order to store the data blocks.
pub struct Irkle<T: AsRef<[u8]>> {
    /// Inner array used as the main storage device for the tree
    pub inner: Vec<Node<T>>,

    /// A list of referenced towards data blocks stored in [Irkle::inner], used
    /// for quick searching and to allow better usability of the raw data
    ///
    /// All [Node]s of this vector are guaranteed to have an [Option::Some] value
    /// for the [Node::data] element.
    pub data: Vec<Weak<Node<T>>>,
}

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
