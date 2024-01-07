// Local imports
use crate::errors::MerkleError;
use crate::hasher::Hasher;
use crate::node::{Node, SerializableData};
use crate::proof::MerkleProof;

// Core lib imports
use std::collections::HashMap;

#[derive(Clone)]
pub struct IndexedMerkleTree<D: SerializableData, H: Hasher> {
  pub root: Node<D>,
  pub leaves: HashMap<(usize, usize), Node<D>>, // (level, index) -> node
  pub height: usize,
  pub indexer: HashMap<[u8; 32], usize>, // key -> index position in `leaves`
  hasher: H,
}

impl<D: SerializableData, H: Hasher> IndexedMerkleTree<D, H> {
  pub fn new(data: Vec<D>, hasher: H) -> Self {
    let mut indexer: HashMap<[u8; 32], usize> = HashMap::new();
    let mut leaves: HashMap<(usize, usize), Node<D>> = HashMap::new();

    // insert leaves into the tree
    data.iter().enumerate().for_each(|(index, data)| {
      let hash = hasher.hash_leaf(&data.to_bytes());

      leaves.insert(
        (0, index),
        Node {
          hash,
          data: Some(data.clone()),
        },
      );
      indexer.insert(data.key(), index);
    });

    // pad to next power of two with empty leaves
    let padding_count = data.len().next_power_of_two() - data.len();
    for i in 0..padding_count {
      leaves.insert(
        (0, data.len() + i),
        Node {
          hash: H::zero(),
          data: None,
        },
      );
    }

    // build the tree by recursively hashing pairs of leaves
    let (root, height) = build_tree(data.len(), &mut leaves, &hasher).unwrap();

    Self {
      root,
      leaves,
      height,
      indexer,
      hasher,
    }
  }

  pub fn get_proof(&self, key: [u8; 32]) -> Result<MerkleProof<D>, MerkleError> {
    let target_index = *self
      .indexer
      .get(&key)
      .ok_or(MerkleError::InvalidKey { key })?;
    let mut index = target_index;
    let target_node = self
      .leaves
      .get(&(0, index))
      .ok_or(MerkleError::NodeNotFound { level: 0, index })?;

    // tree starts bottom up at level 0 (leaves) and goes up to the root (level `height - 1`)
    let mut proof = Vec::new();
    let mut level = 0;

    while level < self.height {
      let sibling_index = get_sibling_node(index);
      let sibling_node =
        self
          .leaves
          .get(&(level, sibling_index))
          .ok_or(MerkleError::NodeNotFound {
            level,
            index: sibling_index,
          })?;
      proof.push(sibling_node.hash);
      (level, index) = get_parent_node(level, index);
    }

    Ok(MerkleProof {
      data: target_node.data.clone().unwrap(),
      index: target_index,
      proof,
      root_hash: self.root.hash,
    })
  }

  pub fn verify_proof(&self, proof: MerkleProof<D>) -> Result<bool, MerkleError> {
    let mut hash = self.hasher.hash_leaf(&proof.data.to_bytes());
    let mut level = 0;
    let mut index = proof.index;

    for sibling_hash in proof.proof {
      hash = if index % 2 == 0 {
        self.hasher.hash_internal(&hash, &sibling_hash)
      } else {
        self.hasher.hash_internal(&sibling_hash, &hash)
      };
      (level, index) = get_parent_node(level, index);
    }

    if hash != proof.root_hash {
      return Err(MerkleError::InvalidRootHash {
        exp: proof.root_hash,
        act: hash,
      });
    }
    if hash != self.root.hash {
      return Err(MerkleError::InvalidRootHash {
        exp: self.root.hash,
        act: hash,
      });
    }

    Ok(true)
  }
}

impl<D: SerializableData, H: Hasher> std::fmt::Debug for IndexedMerkleTree<D, H> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("IndexedMerkleTree")
      .field("root", &format!("0x{}", hex::encode(self.root.hash)))
      .field(
        "leaves",
        &self
          .leaves
          .iter()
          .map(|((level, index), _node)| format!("({},{})", level, index))
          .collect::<Vec<_>>(),
      )
      .field("height", &self.height)
      .field(
        "indexer",
        &self
          .indexer
          .iter()
          .map(|(key, value)| format!("0x{} -> {}", hex::encode(key), value))
          .collect::<Vec<_>>(),
      )
      .finish()
  }
}

fn build_tree<H: Hasher, D: SerializableData>(
  data_len: usize,
  leaves: &mut HashMap<(usize, usize), Node<D>>,
  hasher: &H,
) -> Result<(Node<D>, usize), MerkleError> {
  if data_len < 2 {
    return Err(MerkleError::InvalidDataLength { len: data_len });
  }

  let mut level = 1; // skip level 0 (leaves)
  let mut index = 0;
  let mut max_index = data_len.next_power_of_two() / 2 - 1;
  let height = data_len.next_power_of_two().ilog2() as usize;

  while level <= height {
    while index <= max_index {
      let left_node = get_left_node(level, index);
      let right_node = get_right_node(level, index);
      let left_hash = leaves.get(&left_node).unwrap().hash;
      let right_hash = leaves.get(&right_node).unwrap().hash;
      let hash = hasher.hash_internal(&left_hash, &right_hash);
      leaves.insert((level, index), Node { hash, data: None });
      index += 1;
    }
    level += 1;
    index = 0;
    max_index = max_index / 2;
  }

  // get root node
  let root = leaves
    .get(&(height, 0))
    .ok_or(MerkleError::NodeNotFound {
      level: height,
      index: 0,
    })?
    .clone();

  Ok((root, height))
}

fn get_parent_node(level: usize, index: usize) -> (usize, usize) {
  (level + 1, index / 2)
}

fn get_sibling_node(index: usize) -> usize {
  index ^ 1
}

fn get_left_node(level: usize, index: usize) -> (usize, usize) {
  (level - 1, index * 2)
}

fn get_right_node(level: usize, index: usize) -> (usize, usize) {
  (level - 1, index * 2 + 1)
}
