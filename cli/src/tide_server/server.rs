use http_types::headers::HeaderValue;
use tide::prelude::*;
use tide::security::{CorsMiddleware, Origin};

use super::handlers;
use super::middleware::Auth;

pub struct TideServer {}

impl TideServer {
    pub async fn start(port: u16) -> tide::Result<()> {
        let mut app = tide::new();

        let cors = CorsMiddleware::new()
            .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::from("*"))
            .allow_credentials(false);

        app.with(cors);

        app.at("/").serve_file("resources/hello.html")?;

        app.at("/shoes")
            .get(handlers::hello)
            .post(handlers::order_shoes);
        app.at("/hello").get(handlers::hello);
        app.at("/login").post(handlers::login);
        app.at("/register").post(handlers::register);
        app.at("/secure").with(Auth::new()).get(handlers::secure);
        app.at("/user/:id").get(handlers::user);

        let mut listener = app.bind(format!("127.0.0.1:{}", port)).await?;

        for info in listener.info().iter() {
            println!("Now listening on {}", info);
        }

        listener.accept().await?;
        Ok(())
    }
}
