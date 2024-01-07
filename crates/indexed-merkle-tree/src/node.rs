#[derive(Debug, Clone)]
pub struct Node {
  pub hash: [u8; 32],
  pub data: Option<Vec<u8>>,
}