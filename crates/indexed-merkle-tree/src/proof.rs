// Local imports
use crate::node::SerializableData;

// External imports
use hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MerkleProof<D: SerializableData> {
  pub data: D,
  pub index: usize,
  pub proof: Vec<[u8; 32]>,
  pub root_hash: [u8; 32],
}

impl<D: SerializableData> std::fmt::Debug for MerkleProof<D> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("MerkleProof")
      .field("data", &String::from_utf8_lossy(&self.data.to_bytes()))
      .field("index", &format!("{}", self.index))
      .field("root_hash", &format!("0x{}", hex::encode(self.root_hash)))
      .field(
        "proof",
        &self
          .proof
          .iter()
          .map(|h| format!("0x{}", hex::encode(h)))
          .collect::<Vec<_>>(),
      )
      .finish()
  }
}
