// Local imports
use api::data_parser::{parse_data, DistributionEntry};
use api::endpoints::{get_info, get_proof, status, ApiDoc};
use api::AppState;
use indexed_merkle_tree::{hasher::KeccakHasher, tree::IndexedMerkleTree};

// Core lib imports
use std::sync::Arc;

// External imports
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // parse distribution data
  let (data, total_amount) = parse_data();

  // build merkle tree
  let tree = IndexedMerkleTree::<DistributionEntry, KeccakHasher>::new(data, KeccakHasher);

  // wrap in Arc for thread-safe shared access
  let app_state = Arc::new(AppState { tree, total_amount });

  // set log level
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(app_state.clone()))
      .wrap(Logger::default())
      .wrap(Logger::new("%a %{User-Agent}i"))
      .service(status)
      .service(get_info)
      .service(get_proof)
      .service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
      )
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
