// Local imports
use api::data_parser::{parse_data, DistributionEntry};
use api::endpoints::{status, ApiDoc};
use indexed_merkle_tree::{hasher::KeccakHasher, tree::IndexedMerkleTree};

// External imports
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // parse distribution data
  let data = parse_data();
  println!("data: {:?}", data);

  // build merkle tree
  let tree = IndexedMerkleTree::<DistributionEntry, KeccakHasher>::new(data, KeccakHasher);
  println!("tree: {:#?}", tree);

  // set log level
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .wrap(Logger::new("%a %{User-Agent}i"))
      .service(status)
      .service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
      )
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
