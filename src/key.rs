use super::KEY_LEN;
use sha2::{Digest, Sha256};
use std::array::from_fn;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Key(pub [u8; KEY_LEN]);


impl Key {
    pub fn new(input: String) -> Self {
        let res = Sha256::new().chain_update(input.as_bytes()).finalize();

        let hash = res.as_slice().try_into().expect("Hash length mismatch");

        Self(hash)
    }
    pub fn distance(&self, other: &Key) -> Key {
        let distance = from_fn(|i| self.0[i] ^ other.0[i]);
        Key(distance)
    }
}
