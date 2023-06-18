use tide::{Error, Middleware, Next, Request, Response, StatusCode};

use super::jwt::verify;

pub struct Auth {}

impl Auth {
    pub fn new() -> Self {
        Auth {}
    }
}

// Some references:
// svelte-tide-project - https://github.com/jbertovic/svelte-tide-project/blob/main/src/middleware/usersecure.rs
// tide logger middleware - https://github.com/http-rs/tide/blob/main/src/log/middleware.rs
// tide-jwt - https://github.com/nyxtom/tide-jwt/blob/master/src/lib.rs
#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for Auth {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result<Response, Error> {
        let auth_header = req.header("Authorization");
        // println!("Auth header: {:?}", auth_header);

        if let Some(header) = auth_header {
            let auth: Vec<_> = header.into_iter().collect();
            // println!("Auth before split: {:?}", auth);

            if auth.len() != 1 {
                return Ok(Response::new(StatusCode::Unauthorized));
            }

            let token = &auth[0].as_str()["Bearer ".len()..];
            println!("Auth: {:?}", token);

            return match verify(token.to_string()) {
                Ok(_) => Ok(next.run(req).await),
                Err(_) => Ok(Response::new(StatusCode::Unauthorized)),
            };
        }

        return Ok(Response::new(StatusCode::Unauthorized));
    }
}
