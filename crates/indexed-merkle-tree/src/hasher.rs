use light_poseidon::{Poseidon, PoseidonBytesHasher};
use ark_bn254::Fr;

pub trait Hasher {
  fn hash_leaf(&self, data: &[u8]) -> [u8; 32];
  fn hash_internal(&self, left: &[u8; 32], right: &[u8; 32]) -> [u8; 32];
  fn zero() -> [u8; 32];
}

pub struct PoseidonHasher;

impl Hasher for PoseidonHasher {
  fn hash_leaf(&self, data: &[u8]) -> [u8; 32] {
    let mut poseidon = Poseidon::<Fr>::new_circom(2).unwrap();

    poseidon.hash_bytes_be(&[data]).unwrap()
  }

  fn hash_internal(&self, left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut poseidon = Poseidon::<Fr>::new_circom(2).unwrap();

    poseidon.hash_bytes_be(&[left, right]).unwrap()
  }

  fn zero() -> [u8; 32] {
    [0; 32]
  }
}