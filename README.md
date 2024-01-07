# Merkle Distributor API

This repo implements a reward distribution service for allocating token rewards to a large number of users. Given an input list of addresses and amounts, it builds an in-memory Merkle tree and provides a REST API for requesting proofs and verifying them against tree data.

The root hash can be stored on-chain and used together with a verifier contract to distribute rewards to users efficiently.

The project is organized into the following crates:
- `indexed-merkle-tree`: Merkle tree library with key-value index for O(1) lookup of leaves by their associated key (e.g. user's wallet address)
- `api`: REST API and data parser for building the Merkle tree and hosting the proof request and verification service; also hosts a Swagger UI for OpenAPI documentation

## Usage

To build the project, run:
```bash
cargo build
```

To run tests: 
```bash
cargo test
```

To use the API, first populate `crates/api/data` with the distribution data in JSON format. The data should be in the following format:

```json
[
  {
    "address": "0x1234567890123456789012345678901234567890",
    "amount": "1000000000000000000"
  }
]
```

You can add multiple files to the data directory, for example to support successive rounds of distributions.

To run the API, run:

```bash
cargo run 

# or explicitly target api crate
cargo run -p api
```

The API will be available at: `http://localhost:8080`

You can also view the Swagger UI at: `http://localhost:8080/swagger-ui/#`