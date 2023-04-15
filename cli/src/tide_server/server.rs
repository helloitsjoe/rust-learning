use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

pub struct TideServer {}

impl TideServer {
    pub async fn start(port: u16) -> tide::Result<()> {
        let mut app = tide::new();
        println!("Hello");
        app.at("/shoes").post(order_shoes);
        println!("Hello Two");
        app.listen("127.0.0.1:8080").await?;
        println!("Now listening on port {}", port);
        Ok(())
    }
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've placed an order for {} shoes.", name, legs).into())
}
