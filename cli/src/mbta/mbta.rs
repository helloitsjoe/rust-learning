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

    pub fn start(self, input: &mut Input) {
        println!("Hi");
        let result = self.handle_input(input);
        println!("{:?}", result);
        if let Err(e) = result {
            panic!("{:?}", e);
        }
    }

    pub fn handle_input(self, input: &mut Input) -> Result<(), Box<dyn Error>> {
        println!("Please enter a route");
        let binding = input.get_input().to_lowercase();
        let route = binding.as_str();
        println!("Route: {:?}", route);
        // Request route

        let res = reqwest::blocking::get("https://api-v3.mbta.com/routes/Red")?.text()?;

        println!("{:?}", res);
        let json: MbtaResponse = serde_json::from_str(res.as_str())?;

        println!("{:#?}", json.data);
        return Ok(());
    }
}
