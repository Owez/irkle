use blake3::Hash;

/// Trait for running methods on any abstract kind of node, such as hash verification
/// or just getting the hash
pub trait NodeMethod<T: AsRef<[u8]>> {
    /// Gets the [blake3]-based [Hash] for trait implementation, just call on any
    /// [Node], [Data] or [NodeType] like so: `item.get_hash()`. Typically all
    /// this method will do is get the `self.hash` but this can be used to adapt
    /// a broader [NodeType]
    fn get_hash(&self) -> Hash;

    /// Verifies the node down through recursion, providing a high-level
    /// checking/verification method
    ///
    /// If this fails, it will return the expected hash and the found hash where
    /// this hash failed at; this is formatted as `(expected_hash, found_node)`
    fn verify(&self) -> Result<(), (Hash, Hash)>;
}
