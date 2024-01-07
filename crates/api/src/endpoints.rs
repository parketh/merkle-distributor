// Local imports
use crate::data_parser::DistributionEntry;
use crate::AppState;
use indexed_merkle_tree::hasher::{Hasher, KeccakHasher};
use indexed_merkle_tree::proof::MerkleProof;

// Core lib imports
use std::sync::Arc;

// External imports
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(
      status,
      get_info,
      get_proof,
      verify_proof
    ),
    tags(
        (name = "Merkle distributor API", description = "API to request Merkle proofs for reward distribution.")
    ),
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "/",
  responses(
    (status = 200, description = "Status"),
  )
)]
#[get("/")]
async fn status() -> impl Responder {
  HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[utoipa::path(
  get,
  path = "/info",
  responses(
    (status = 200, description = "Return the total amount of rewards and Merkle root hash"),
  )
)]
#[get("/info")]
async fn get_info(app_state: web::Data<Arc<AppState>>) -> impl Responder {
  let root_hash = app_state.tree.root.hash;
  HttpResponse::Ok().json(serde_json::json!({
    "total_amount": app_state.total_amount,
    "root_hash": format!("0x{}", hex::encode(root_hash)),
  }))
}

#[derive(Deserialize, IntoParams)]
struct ProofQuery {
  address: String,
}

#[derive(Serialize)]
struct ProofResponse {
  amount: String,
  proof: Vec<String>,
}

#[utoipa::path(
  get,
  path = "/proof",
  params(
    ProofQuery
  ),
  responses(
    (status = 200, description = "Request Merkle proof for a given address"),
  )
)]
#[get("/proof")]
async fn get_proof(
  app_state: web::Data<Arc<AppState>>,
  query: web::Query<ProofQuery>,
) -> impl Responder {
  let key = KeccakHasher.hash_leaf(&query.address.as_bytes());
  match app_state.tree.get_proof(key) {
    Ok(proof) => {
      let formatted = ProofResponse {
        amount: proof.data.amount,
        proof: proof
          .proof
          .iter()
          .map(|h| format!("0x{}", hex::encode(h)))
          .collect(),
      };
      HttpResponse::Ok().json(serde_json::json!(formatted))
    }
    _ => HttpResponse::InternalServerError().body("Failed to get proof for address."),
  }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VerifyProofQuery {
  pub address: String,
  pub amount: String,
  pub proof: Vec<String>,
}

#[derive(Serialize)]
struct VerifyProofResponse {
  valid: bool,
}

#[utoipa::path(
  post,
  path = "/verify",
  request_body(
    content = VerifyProofQuery,
    content_type = "application/json",
  ),
  responses(
    (status = 200, description = "Verify Merkle proof for a given address and amount"),
  )
)]
#[post("/verify")]
async fn verify_proof(
  app_state: web::Data<Arc<AppState>>,
  body: web::Json<VerifyProofQuery>,
) -> impl Responder {
  let proof: MerkleProof<DistributionEntry> = MerkleProof {
    data: DistributionEntry {
      address: body.address.clone(),
      amount: body.amount.clone(),
    },
    proof: body
      .proof
      .iter()
      .map(|h| {
        let bytes = hex::decode(h.trim_start_matches("0x")).unwrap();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes[..32]);
        arr
      })
      .collect(),
  };
  match app_state.tree.verify_proof(proof) {
    Ok(valid) => HttpResponse::Ok().json(VerifyProofResponse { valid }),
    _ => HttpResponse::InternalServerError().body("Failed to verify proof."),
  }
}
