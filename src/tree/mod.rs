//! Contains [Tree] and related implementations, see item-level documentation for
//! more information

mod new;
mod retrieval;

pub use new::*;
pub use retrieval::*;

use crate::Node;
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
#[derive(Debug, Clone)]
pub struct Tree<T: AsRef<[u8]>> {
    /// Inner array used as the main storage device for the tree
    pub inner: Vec<Node<T>>,

    /// A list of referenced towards data blocks stored in [Tree::inner], used
    /// for quick searching and to allow better usability of the raw data
    ///
    /// All [Node]s of this vector are guaranteed to have an [Option::Some] value
    /// for the [Node::data] element.
    pub data: Vec<Weak<Node<T>>>,
}
