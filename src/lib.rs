//! A merkle tree implementation based upon the [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)
//! hash and [binary tree arrays](https://en.wikipedia.org/wiki/Binary_tree#Arrays)
//! for superfast trees ⚡
//!
//! Please travel to the primary [Irkle] structure of this library for more
//! documentation then this module-level infomation can provide, as it is the main
//! implemented utility that this library aims to provide.

mod irkle;
mod node;

pub use irkle::Irkle;
pub use node::Node;
