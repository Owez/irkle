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
/// # Examples
///
/// A simple tree being created:
///
/// ```rust
/// use irkle::Tree;
///
/// fn main() {
///     let data = vec!["foxtrot", "uniform", "charlie", "kilo"];
///     let tree = Tree::new(data);
///
///     println!("{:?}", tree);
/// }
/// ```
///
/// ***More coming soon..***
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
    /// Inner array used as the main storage device for the tree, containing a
    /// form of [binary tree arrays](https://en.wikipedia.org/wiki/Binary_tree#Arrays)
    ///
    /// If you'd like a usable list of all accessible data blocks inside of the
    /// tree, you may want to utilise the attached [Tree::data] element, which
    /// allows direct access.
    pub inner: Vec<Node<T>>,

    /// A list of referenced towards data blocks stored in [Tree::inner] and their
    /// position inside said array, used for quick searching and to allow better
    /// usability of the raw data
    ///
    /// All [Node]s of this vector are guaranteed to have an [Option::Some] value
    /// for the [Node::inner] element
    ///
    /// # Behind-the-scenes
    ///
    /// This element simply makes a new [Vec] of the elements of:
    ///
    /// ```none
    /// (inner.len() / 2 + 1)..array.len()
    /// ```
    ///
    /// Due to [binary tree arrays](https://en.wikipedia.org/wiki/Binary_tree#Arrays)
    /// simply having all data blocks at the end of the array, starting from the
    /// previously mentioned `inner.len() / 2 + 1` equation
    pub data: Vec<(Weak<Node<T>>, usize)>,
}
