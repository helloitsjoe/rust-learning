use http_types::headers::HeaderValues;
use tide::prelude::*;
use tide::Request;

// use http::headers::HeaderValue;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

pub struct TideServer {}

// sub routes
// headers
// middleware/auth
// cors

impl TideServer {
    pub async fn start(port: u16) -> tide::Result<()> {
        let mut app = tide::new();
        app.at("/")
            .get(|_| async { Ok(tide::Redirect::new("/shoes")) });

        app.at("/shoes").get(hello).post(order_shoes);

        let mut listener = app.bind(format!("127.0.0.1:{}", port)).await?;

        for info in listener.info().iter() {
            println!("Now listening on {}", info);
        }

        listener.accept().await?;
        Ok(())
    }
}

async fn hello(req: Request<()>) -> tide::Result {
    let name = req.param("name").unwrap_or("Anonymous");
    Ok(format!(
        "Hello, {}! Order some shoes with a post request! Just let me know how may legs you have.",
        name
    )
    .into())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    // Having some trouble typing unwrap_or("default")
    // let Headers { foo } = req.header_values();
    // let foo = req.header("x-foo").unwrap_or_default().to_string();
    // let maybe_foo = req.header("x-foo");

    let response = tide::Response::builder(200)
        .header("x-foo-response", "bar")
        .body(format!(
            "Hello, {}! I've placed an order for {} shoes.",
            name, legs
        ))
        .build();

    Ok(response)
}
