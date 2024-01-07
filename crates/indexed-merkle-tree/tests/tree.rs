use indexed_merkle_tree::hasher::{Hasher, KeccakHasher};
use indexed_merkle_tree::node::SerializableData;
use indexed_merkle_tree::tree::IndexedMerkleTree;

#[derive(Debug, Clone)]
struct TestData(String);

impl SerializableData for TestData {
  fn to_bytes(&self) -> Vec<u8> {
    self.0.as_bytes().to_vec()
  }

  fn from_bytes(bytes: Vec<u8>) -> Self {
    TestData(String::from_utf8(bytes).unwrap())
  }

  fn key(&self) -> [u8; 32] {
    KeccakHasher.hash_leaf(&self.0.as_bytes())
  }
}

#[test]
fn test_tree() {
  let data: Vec<TestData> = vec![
    TestData("hello".to_string()),
    TestData("world".to_string()),
    TestData("foo".to_string()),
    TestData("bar".to_string()),
    TestData("baz".to_string()),
  ];

  let tree = IndexedMerkleTree::<TestData, KeccakHasher>::new(data, KeccakHasher);
  let proof = tree.get_proof(TestData("hello".to_string()).key()).unwrap();
  println!("proof: {:#?}", proof);
  tree.verify_proof(proof).unwrap();
}
