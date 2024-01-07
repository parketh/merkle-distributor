#[derive(Debug)]
pub enum MerkleError {
  InvalidRootHash { exp: [u8; 32], act: [u8; 32] },
  InvalidKey { key: [u8; 32] },
  NodeNotFound { level: usize, index: usize },
  InvalidDataLength { len: usize },
}
