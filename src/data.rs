use crate::NodeMethod;
use blake3::Hash;

/// The final datablock, containing the data needed
#[derive(Debug, Clone, PartialEq)]
pub struct Data<T: AsRef<[u8]>> {
    pub hash: Hash,
    pub data: T,
}

impl<T: AsRef<[u8]>> Data<T> {
    /// Creates a new [Data] from given `data`
    pub fn new<D: Into<T>>(data: D) -> Self {
        let data_into = data.into();

        Self {
            hash: blake3::hash(data_into.as_ref()),
            data: data_into.into(),
        }
    }
}

impl<T: AsRef<[u8]>> NodeMethod<T> for Data<T> {
    fn get_hash(&self) -> Hash {
        self.hash
    }

    fn verify(&self) -> Result<(), (Hash, Hash)> {
        let found_hash = blake3::hash(self.data.as_ref());

        if self.hash == found_hash {
            Ok(())
        } else {
            Err((found_hash, self.hash))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeType;

    const TEST_DATA: &str = "hello";

    #[test]
    fn data_to_node_type() {
        let inner: Data<&str> = Data::new("");
        assert_eq!(NodeType::from(inner.clone()), NodeType::Data(inner))
    }

    #[test]
    fn data_get_hash() {
        let data: Data<&str> = Data::new(TEST_DATA);
        assert_eq!(data.get_hash(), blake3::hash(TEST_DATA.as_bytes()));
    }

    #[test]
    fn data_verification() {
        let mut test_struct: Data<&str> = Data::new(TEST_DATA);
        assert!(test_struct.verify().is_ok());

        test_struct.hash = blake3::hash(b"fknrejnfjrenf");
        assert!(test_struct.verify().is_err());
    }
}
