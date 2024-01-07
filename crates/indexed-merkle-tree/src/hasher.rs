use sha3::{Digest, Keccak256};

pub trait Hasher {
  fn hash_leaf(&self, data: &[u8]) -> [u8; 32];
  fn hash_internal(&self, left: &[u8; 32], right: &[u8; 32]) -> [u8; 32];
  fn zero() -> [u8; 32];
}

pub struct KeccakHasher;

impl Hasher for KeccakHasher {
  fn hash_leaf(&self, data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
  }

  fn hash_internal(&self, left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
  }

  fn zero() -> [u8; 32] {
    [0; 32]
  }
}
