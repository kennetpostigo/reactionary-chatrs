use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::Server;
use crate::State;

pub fn cors_middleware(mut app: Server<State>) -> Server<State> {
  let cors = CorsMiddleware::new()
    .allow_methods(
      "GET, POST, PUT, DELETE, OPTIONS"
        .parse::<HeaderValue>()
        .unwrap(),
    )
    .allow_origin(Origin::from("*"))
    .allow_credentials(true);
  app.with(cors);

  app
}
