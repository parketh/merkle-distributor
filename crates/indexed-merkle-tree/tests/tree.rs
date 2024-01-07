use indexed_merkle_tree::hasher::{Hasher, KeccakHasher};
use indexed_merkle_tree::tree::IndexedMerkleTree;

#[test]
fn test_tree() {
  let data = vec![
    b"hello".to_vec(),
    b"world".to_vec(),
    b"foo".to_vec(),
    b"bar".to_vec(),
    b"baz".to_vec(),
  ];

  let tree = IndexedMerkleTree::new(data, KeccakHasher);
  let proof = tree.get_proof(KeccakHasher.hash_leaf(b"hello")).unwrap();
  println!("proof: {:#?}", proof);
  tree.verify_proof(proof).unwrap();
}
