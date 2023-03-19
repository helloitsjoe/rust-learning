use super::super::Input;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

pub struct MBTA {}

#[derive(Deserialize)]
struct MbtaResponse {
    data: Value,
}

impl MBTA {
    pub fn new() -> MBTA {
        MBTA {}
    }

    pub async fn start(self, input: &mut Input) {
        let result = self.handle_input(input).await;
        if let Err(e) = result {
            panic!("{:?}", e);
        }
    }

    pub async fn handle_input(self, input: &mut Input) -> Result<(), Box<dyn Error>> {
        println!("Please enter a route");
        let binding = input.get_input().to_lowercase();
        let route = binding.as_str();
        println!("Route: {:?}", route);

        let res = reqwest::get("https://api-v3.mbta.com/routes/".to_owned() + route)
            .await?
            .text()
            .await?;

        let json: MbtaResponse = serde_json::from_str(res.as_str())?;

        println!("{:#?}", json.data);
        return Ok(());
    }
}
