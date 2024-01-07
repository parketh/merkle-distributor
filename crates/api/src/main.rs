pub mod endpoints;

use crate::endpoints::{status, ApiDoc};

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
