use serde::Deserialize;
use std::io::BufReader;

const BASE_URL: &str = "https://hackattic.com/challenges/brute_force_zip/problem";

#[derive(Deserialize)]
struct Response {
    zip_url: String,
}

pub fn brute_force() -> Result<(), reqwest::Error> {
    let access_token: String = std::env::var("ACCESS_TOKEN").expect("Access token not set");

    println!("token {:?}", access_token);

    let Response { zip_url } =
        reqwest::blocking::get(format!("{}{}{}", BASE_URL, "?access_token=", access_token))?
            .json()?;
    println!("{:?}", zip_url);

    let zip_file = reqwest::blocking::get(zip_url)?.bytes()?;
    // println!("{:?}", zip_file);
    // let reader = BufReader::new(zip_file);

    Ok(())
}
