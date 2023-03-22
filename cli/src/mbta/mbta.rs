use super::super::Input;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

pub struct MBTA {}

#[derive(Deserialize, Debug)]
struct RouteAttrs {
    direction_destinations: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct RouteData {
    attributes: RouteAttrs,
}

#[derive(Deserialize)]
struct RouteResponse {
    data: RouteData,
}

#[derive(Deserialize, Debug)]
struct StopsAttrs {
    name: String,
}

#[derive(Deserialize, Debug)]
struct StopsData {
    attributes: StopsAttrs,
    id: String,
}

#[derive(Deserialize, Debug)]
struct StopsResponse {
    data: Vec<StopsData>,
}

#[derive(Deserialize, Debug)]
struct PredictionsAttrs {
    departure_time: String,
}

#[derive(Deserialize, Debug)]
struct PredictionsData {
    attributes: PredictionsAttrs,
}

#[derive(Deserialize, Debug)]
struct PredictionsResponse {
    data: Vec<PredictionsData>,
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
        println!("Please enter a route:");
        let route = input.get_input().to_lowercase();
        // println!("Route: {:?}", route);

        let directions = reqwest::get(format!("https://api-v3.mbta.com/routes/{}", &route))
            .await?
            .json::<RouteResponse>()
            .await?;

        // println!("{:?}", res);

        println!("Choose a direction:");
        for (i, dest) in directions
            .data
            .attributes
            .direction_destinations
            .iter()
            .enumerate()
        {
            println!("{}", format!("{} - {}", i + 1, dest));
        }

        let direction: u32 = input.get_input().parse()?;
        let direction = direction - 1;
        // println!("Direction: {:?}", direction);

        let stops = reqwest::get(format!(
            "https://api-v3.mbta.com/stops?filter[route]={}&filter[direction_id]={}",
            &route, direction
        ))
        .await?
        .json::<StopsResponse>()
        .await?;

        // println!("Stops: {:?}", stops);

        println!("Choose a stop:");
        for (i, stop) in stops.data.iter().enumerate() {
            println!("{}", format!("{} - {}", i + 1, stop.attributes.name));
        }

        let stop: u32 = input.get_input().parse()?;
        let stop_idx = (stop - 1) as usize;
        let stop_id = stops.data[stop_idx].id.clone();
        // println!("{:?}", stop_id);

        let predictions = reqwest::get(format!(
            "https://api-v3.mbta.com/predictions?filter[stop]={}",
            stop_id
        ))
        .await?
        .json::<PredictionsResponse>()
        .await?;

        println!("Next departures at:");
        for prediction in predictions.data.iter() {
            // TODO: Handle null values
            println!("{}", prediction.attributes.departure_time);
        }

        // println!("{:#?}", predictions.data);
        return Ok(());
    }
}
