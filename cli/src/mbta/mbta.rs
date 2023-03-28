use super::super::Input;
use chrono::prelude::*;
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

#[derive(Deserialize, Debug)]
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
    departure_time: Option<String>,
}

#[derive(Deserialize, Debug)]
struct PredictionsData {
    attributes: PredictionsAttrs,
    // attributes: Value,
}

#[derive(Deserialize, Debug)]
struct PredictionsResponse {
    data: Vec<PredictionsData>,
}

const MBTA_API: &str = "https://api-v3.mbta.com";

async fn fetch_mbta<T: for<'de> Deserialize<'de>>(url: String) -> Result<T, reqwest::Error> {
    reqwest::get(url).await?.json::<T>().await
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
        let route = input.get_input();

        // TODO: Handle route capitalization
        let directions = fetch_mbta::<RouteResponse>(format!("{MBTA_API}/routes/{route}")).await?;

        println!("{:?}", directions);

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
        let direction_id = direction - 1;

        let stops = fetch_mbta::<StopsResponse>(format!(
            "{MBTA_API}/stops?filter[route]={route}&filter[direction_id]={direction_id}"
        ))
        .await?;

        // println!("Stops: {:?}", stops);

        println!("Choose a stop:");
        for (i, stop) in stops.data.iter().enumerate() {
            println!("{}", format!("{} - {}", i + 1, stop.attributes.name));
        }

        let stop: u32 = input.get_input().parse()?;
        let stop_idx = (stop - 1) as usize;
        let stop_id = stops.data[stop_idx].id.clone();

        let predictions = fetch_mbta::<PredictionsResponse>(format!(
            "{MBTA_API}/predictions?filter[stop]={stop_id}&filter[route]={route}&filter[direction_id]={direction}",
        ))
        .await?;

        // TODO: Parse dates
        println!("Next departures at:");
        for prediction in predictions.data.iter() {
            let departure_time = &prediction.attributes.departure_time;
            if let Some(dep_time) = departure_time {
                println!("{:?}", dep_time);
                // let dt = Utc.with_ymd_and_hms(dep_time).unwrap();
                let parsed = DateTime::parse_from_rfc3339(dep_time).unwrap();
                println!("{:?}", parsed.format("%H:%M").to_string());
            }
        }

        // println!("{:#?}", predictions.data);
        return Ok(());
    }
}
