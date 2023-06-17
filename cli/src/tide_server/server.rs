use http_types::headers::HeaderValue;
use tide::prelude::*;
use tide::security::{CorsMiddleware, Origin};
use tide::{Body, Request, Response};

use super::middleware::Auth;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: Option<u8>,
}

pub struct TideServer {}

// sub routes
// headers
// middleware/auth
// cors

impl TideServer {
    pub async fn start(port: u16) -> tide::Result<()> {
        let mut app = tide::new();

        let cors = CorsMiddleware::new()
            .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::from("*"))
            .allow_credentials(false);

        app.with(cors);

        app.at("/").serve_file("resources/hello.html")?;

        app.at("/shoes").get(hello).post(order_shoes);
        app.at("/hello").get(hello);
        app.at("/login").post(login);
        app.at("/register").post(register);
        app.at("/secure").with(Auth::new()).get(hello);
        app.at("/user/:id").get(user);

        let mut listener = app.bind(format!("127.0.0.1:{}", port)).await?;

        for info in listener.info().iter() {
            println!("Now listening on {}", info);
        }

        listener.accept().await?;
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(default)]
struct Person {
    name: String,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Anonymous".to_string(),
        }
    }
}

async fn hello(req: Request<()>) -> tide::Result {
    let person: Person = req.query()?;
    Ok(format!(
        "Hello, {}! Order some shoes with a post request! Just let me know how may legs you have.",
        person.name
    )
    .into())
}

async fn user(req: Request<()>) -> tide::Result {
    let id = req.param("id").unwrap();
    Ok(format!("User {}", id).into())
}

#[derive(Debug, Deserialize)]
struct Login {
    name: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
}

fn sign_jwt() {}

fn validate_login(name: String, password: String) -> String {
    // Check in-memory users
    // return JWT
    "foo".to_string()
}

async fn register(mut req: Request<()>) -> tide::Result {
    let Login { name, password } = req.body_json().await?;

    // Add new user to in-memory users
    // return JWT

    let response = Response::builder(200)
        .body("some-token".to_string())
        .build();
    Ok(response)
}

async fn login(mut req: Request<()>) -> tide::Result {
    let Login { name, password } = req.body_json().await?;
    println!("{:?}, {:?}", name, password);

    let token = validate_login(name, password);

    let login_response = LoginResponse {
        token: "some-token".to_string(),
    };

    let response = Response::builder(200)
        .body(Body::from_json(&login_response)?)
        .build();
    Ok(response)
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;

    // TODO: Type validation
    if legs.is_none() {
        let response = Response::builder(422).body("Legs must be provided").build();
        return Ok(response);
    }

    // Having some trouble typing unwrap_or("default")
    // let Headers { foo } = req.header_values();
    // let foo = req.header("x-foo").unwrap_or_default().to_string();
    // let maybe_foo = req.header("x-foo");

    let response = tide::Response::builder(200)
        .header("x-foo-response", "bar")
        .body(format!(
            "Hello, {}! I've placed an order for {:?} shoes.",
            name,
            legs.unwrap_or(0)
        ))
        .build();

    Ok(response)
}
