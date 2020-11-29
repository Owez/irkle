// THIS SECTION MIRRORS README

//! [![Tests](https://github.com/Owez/irkle/workflows/Tests/badge.svg)](https://github.com/Owez/irkle/actions?query=workflow%3ATests)
//! [![Docs](https://docs.rs/irkle/badge.svg)](https://docs.rs/irkle)
//!
//! A merkle tree library using [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)
//! and [binary tree arrays](https://en.wikipedia.org/wiki/Binary_tree#Arrays) for
//! superfast trees ⚡
//!
//! ## Example
//!
//! ***Coming soon***
//!
//! ## Installation
//!
//! Simply add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [depedencies]
//! irkle = "0.1" # NOTE: not released just yet!
//! ```
//!
//! ## Objectives
//!
//! This project has explicit objectives that are stuck to whilst developing and
//! maintaining:
//!
//! - __High preformance__
//! - Heavily documented functions, should read as semi-documentation, semi-tutorial
//!   on merkle trees
//! - Low-as-possible dependency count, only current dependency is the `blake3`
//!   hasing library

// END OF README MIRROR SECTION

//! # Project links
//!
//! Some links to find this project outside of the currently viewed documentation:
//!
//! - [Repository](https://github.com/owez/irkle)
//! - [Crates.io](https://crates.io/crates/irkle)
//!
//! # Further reading
//!
//! Please travel to the primary [Tree] structure of this library for more
//! documentation then this module-level infomation can provide, as it is the main
//! implemented utility that this library aims to provide.

mod node;
mod tree;

pub use node::Node;
pub use tree::Tree;
