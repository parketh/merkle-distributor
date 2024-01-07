use hex;

pub struct MerkleProof {
  pub data: Vec<u8>,
  pub index: usize,
  pub proof: Vec<[u8; 32]>,
  pub root_hash: [u8; 32],
}

impl std::fmt::Debug for MerkleProof {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("MerkleProof")
      .field("data", &String::from_utf8_lossy(&self.data))
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
