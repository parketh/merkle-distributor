#[derive(Debug, Clone)]
pub struct Node<D: SerializableData> {
  pub hash: [u8; 32],
  pub data: Option<D>,
}

pub trait SerializableData: Clone {
  fn to_bytes(&self) -> Vec<u8>;
  fn from_bytes(bytes: Vec<u8>) -> Self;
  fn key(&self) -> [u8; 32];
}
