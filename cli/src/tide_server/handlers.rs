use tide::prelude::*;
use tide::{Body, Request, Response};

use super::jwt::sign;

#[derive(Debug, Deserialize)]
pub struct Animal {
    name: String,
    legs: Option<u8>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Person {
    name: String,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Anonymous".to_string(),
        }
    }
}

pub async fn hello(req: Request<()>) -> tide::Result {
    let person: Person = req.query()?;

    let foo_header = req
        .header("x-foo")
        .map(|h| h.as_str())
        .unwrap_or("no foo header");

    println!("{:?}", foo_header);

    let response = Response::builder(200)
        .header("x-foo-echo", foo_header)
        .body(format!(
        "Hello, {}! Order some shoes with a post request! Just let me know how may legs you have.",
        person.name
    ))
        .build();

    Ok(response)
}

pub async fn user(req: Request<()>) -> tide::Result {
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

fn validate_login(name: String, password: String) -> String {
    // Check in-memory users
    // return JWT
    "foo".to_string()
}

pub async fn register(mut req: Request<()>) -> tide::Result {
    let Login { name, password } = req.body_json().await?;

    // Add new user to in-memory users
    // return JWT
    let token = sign(name)?;
    let body = Body::from_json(&LoginResponse { token })?;

    let response = Response::builder(200).body(body).build();
    Ok(response)
}

pub async fn login(mut req: Request<()>) -> tide::Result {
    let Login { name, password } = req.body_json().await?;
    println!("{:?}, {:?}", name, password);

    // TODO: Check name/pw against in-memory DB

    let token = sign(name)?;
    let body = Body::from_json(&LoginResponse { token })?;

    let response = Response::builder(200).body(body).build();
    Ok(response)
}

pub async fn secure(req: Request<()>) -> tide::Result {
    let token = req.header("x-token").unwrap();

    Ok(Response::builder(200)
        .header("x-token", token)
        .body("Authorized!")
        .build())
}

pub async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;

    // TODO: Type validation
    if legs.is_none() {
        let response = Response::builder(422).body("Legs must be provided").build();
        return Ok(response);
    }

    let response = Response::builder(200)
        .body(format!(
            "Hello, {}! I've placed an order for {:?} shoes.",
            name,
            legs.unwrap_or(0)
        ))
        .build();

    Ok(response)
}
