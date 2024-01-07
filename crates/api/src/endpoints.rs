use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
      status
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
