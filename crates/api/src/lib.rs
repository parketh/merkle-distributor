pub mod data_parser;
pub mod endpoints;

use data_parser::DistributionEntry;
use indexed_merkle_tree::{hasher::KeccakHasher, tree::IndexedMerkleTree};

// Application state containing the merkle tree
pub struct AppState {
  pub tree: IndexedMerkleTree<DistributionEntry, KeccakHasher>,
}
