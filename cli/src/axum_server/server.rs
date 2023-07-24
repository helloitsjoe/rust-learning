use super::handlers;
use axum::{routing::get, Router};
use http::Method;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub struct AxumServer {}

// query params
// sub routes
// headers
// middleware/auth
// status codes
// post body

impl AxumServer {
    pub async fn new(port: u16) -> Result<AxumServer, Box<dyn std::error::Error>> {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::GET, Method::POST])
            .allow_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ]);

        let app = Router::new()
            .layer(cors)
            .route("/", get(handlers::root))
            .route("/about", get(handlers::about))
            .route("/user/:id", get(handlers::user));

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        println!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(AxumServer {})
    }
}
