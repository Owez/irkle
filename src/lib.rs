//! A merkle tree library using [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)
//! and [binary tree arrays](https://en.wikipedia.org/wiki/Binary_tree#Arrays) for
//! superfast trees ⚡
//!
//! Please travel to the primary [Tree] structure of this library for more
//! documentation then this module-level infomation can provide, as it is the main
//! implemented utility that this library aims to provide.

mod node;
mod tree;

pub use node::Node;
pub use tree::Tree;
