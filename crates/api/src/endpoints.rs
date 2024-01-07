// Local imports
use crate::AppState;

// Core lib imports
use std::sync::Arc;

// External imports
use actix_web::{get, web, HttpResponse, Responder};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
      status,
      get_root
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
  path = "/get-root",
  responses(
    (status = 200, description = "Root hash of the Merkle tree"),
  )
)]
#[get("/get-root")]
async fn get_root(app_state: web::Data<Arc<AppState>>) -> impl Responder {
  let root_hash = app_state.tree.root.hash;
  let encoded = hex::encode(root_hash);
  HttpResponse::Ok().body(format!("0x{}", encoded))
}