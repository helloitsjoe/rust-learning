use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

pub struct TideServer {}

// query params
// sub routes
// headers
// middleware/auth
// cors
// status codes
// post body

impl TideServer {
    pub async fn start(port: u16) -> tide::Result<()> {
        let mut app = tide::new();
        app.at("/")
            .get(|_| async { Ok(tide::Redirect::new("/shoes")) });

        app.at("/shoes")
            .get(|_| async { Ok("Order some shoes with a post request! Give it a name and how may legs you have.") })
            .post(order_shoes);

        let mut listener = app.bind(format!("127.0.0.1:{}", port)).await?;

        for info in listener.info().iter() {
            println!("Now listening on {}", info);
        }

        listener.accept().await?;
        Ok(())
    }
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've placed an order for {} shoes.", name, legs).into())
}
