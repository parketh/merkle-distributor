// Local imports
use indexed_merkle_tree::hasher::{Hasher, KeccakHasher};
use indexed_merkle_tree::node::SerializableData;

// Core lib imports
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// External imports
use primitive_types::U256;
use serde::{Deserialize, Serialize};

const DATA_DIR: &str = "crates/api/data";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DistributionEntry {
  pub address: String,
  pub amount: String,
}

impl SerializableData for DistributionEntry {
  fn to_bytes(&self) -> Vec<u8> {
    // to support arbitrary length data, we encode the length in bytes before each value
    let mut bytes = Vec::new();

    // encode address
    bytes.extend_from_slice(&(self.address.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&self.address.as_bytes());

    // encode amount
    bytes.extend_from_slice(&(self.amount.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&self.amount.as_bytes());

    bytes
  }

  fn from_bytes(bytes: Vec<u8>) -> Self {
    let mut cursor = 0;

    // parse address
    let address_len = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
    cursor += 4;
    let address = String::from_utf8(bytes[cursor..cursor + address_len].to_vec()).unwrap();
    cursor += address_len;

    // parse amount
    let amount_len = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
    cursor += 4;
    let amount = String::from_utf8(bytes[cursor..cursor + amount_len].to_vec()).unwrap();

    DistributionEntry { address, amount }
  }

  fn key(&self) -> [u8; 32] {
    KeccakHasher.hash_leaf(&self.address.as_bytes())
  }
}

pub fn parse_data() -> (Vec<DistributionEntry>, String) {
  let mut entries: HashMap<String, String> = HashMap::new();
  let mut total_amount: U256 = U256::from(0);

  let files = gather_json_files(DATA_DIR);
  for file in files {
    let data = parse_entries(&file);
    for entry in data {
      let amount = U256::from_dec_str(&entry.amount).unwrap();
      if entries.contains_key(&entry.address) {
        let new_amount = U256::from_dec_str(entries.get(&entry.address).unwrap()).unwrap() + amount;
        *entries.get_mut(&entry.address).unwrap() = new_amount.to_string();
      } else {
        entries.insert(entry.address, entry.amount.to_string());
      }
      total_amount += amount;
    }
  }

  // convert hashmap to vec
  let mut result: Vec<DistributionEntry> = entries
    .into_iter()
    .map(|(address, amount)| DistributionEntry { address, amount })
    .collect();
  // sort by address for deterministic ordering
  result.sort_by(|a, b| a.address.cmp(&b.address));

  (result, total_amount.to_string())
}

fn gather_json_files(path: &str) -> Vec<String> {
  let path = Path::new(path);
  let mut files = Vec::new();
  for file in path.read_dir().expect("Failed to read data directory") {
    if let Ok(file) = file {
      if file.path().extension().unwrap_or_default() == "json" {
        files.push(file.path().to_string_lossy().to_string());
      }
    }
  }
  files
}

fn parse_entries(file: &str) -> Vec<DistributionEntry> {
  let data = File::open(file).expect("Failed to open file");
  let reader = BufReader::new(data);
  let entries: Vec<DistributionEntry> =
    serde_json::from_reader(reader).expect("Failed to parse entries");
  entries
}
