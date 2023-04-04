use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

pub struct AxumServer {}

// query params
// sub routes
// headers
// middleware/auth
// cors
// status codes
// post body

impl AxumServer {
    pub async fn new(port: u16) -> AxumServer {
        let app = Router::new().route("/", get(root));

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        println!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

        AxumServer {}
    }
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}
