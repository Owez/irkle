//! Contains [Tree] creation implementations

use super::Tree;

impl<T: AsRef<[u8]>> Tree<T> {
    /// Creates a new [Tree] from any given data, which is automatically hashed
    /// and therefore verified
    ///
    /// # Example
    ///
    /// ```rust
    /// use irkle::Tree;
    ///
    /// fn main() {
    ///     let data = vec!["alpha", "bravo", "charlie"];
    ///     let tree = Tree::new(data);
    ///
    ///     println!("Made tree:\n\n{:?}", tree);
    /// }
    /// ```
    pub fn new() -> Self {
        unimplemented!()
    }
}
