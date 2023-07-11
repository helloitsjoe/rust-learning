use dtmf::decoder::Decoder;
use rodio::{Decoder as RDecoder, Source};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Seek};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use zip::ZipArchive;

const BASE_URL: &str = "https://hackattic.com/challenges/touch_tone_dialing/problem";
const SOLVE_URL: &str = "https://hackattic.com/challenges/touch_tone_dialing/solve";

#[derive(Deserialize)]
struct Response {
    wav_url: String,
}

// fn dtmf(file: &mut File) -> String {
fn dtmf(file_path: &str, file: &mut File) -> String {
    // Load in our audio samples
    // let (header, data) = wav::read(file).unwrap();
    // This can also be done in real time from the sound card
    // let data = data.try_into_sixteen().unwrap();
    // let data = data
    //     .try_into_sixteen()
    //     .unwrap()
    //     .iter()
    //     .map(|e| *e as f32 / i16::MAX as f32)
    //     .collect::<Vec<f32>>();

    // let mut reader = hound::WavReader::open(file_path).unwrap();
    // let data = reader
    //     .samples::<i16>()
    //     .map(|s| s.unwrap() as f32 / i16::MAX as f32)
    //     .collect::<Vec<f32>>();

    let file = BufReader::new(File::open(file_path).unwrap());
    let source = RDecoder::new(file).unwrap();
    let samples = source.convert_samples();
    let sample_rate = samples.sample_rate();
    let data: Vec<f32> = samples.collect();

    println!("{:?}", sample_rate);
    println!("{:?}", data[0]);

    // let mut chunks: Vec<Vec<f32>> = vec![];
    // let mut curr_chunk: Vec<f32> = vec![];
    // let mut prev: f32 = 0.0;

    // for &curr in &data {
    //     if curr != 0.0 {
    //         curr_chunk.push(curr);
    //     } else if prev != 0.0 && curr == 0.0 {
    //         chunks.push(curr_chunk);
    //         curr_chunk = vec![];
    //     }
    //     prev = curr;
    // }

    // println!("chunks {:?}", chunks);

    let mut actual = vec![];

    // set up our decoder
    let mut decoder = Decoder::new(sample_rate, |tone, state| {
        println!("{:?}: {:?}", tone, state);
        actual.push((tone, state));
    });

    // for chunk in chunks {
    decoder.process(&data[..]);
    // }

    println!("actual {:?}", actual);

    return "foo".to_string();
}

pub fn touch_tone() -> Result<(), reqwest::Error> {
    let access_token: String = std::env::var("ACCESS_TOKEN").expect("Access token not set");

    println!("token {:?}", access_token);

    let Response { wav_url } =
        reqwest::blocking::get(format!("{}{}{}", BASE_URL, "?access_token=", access_token))?
            .json()?;

    println!("{:?}", wav_url);

    let wav_bytes = reqwest::blocking::get(wav_url)?.bytes()?;
    // println!("{:?}", wav_bytes);

    let file_path = "./temp/file.wav";

    let _ = fs::write(file_path, wav_bytes);

    let mut file = fs::File::open(file_path).expect("File should exist");

    let sequence = dtmf(file_path, &mut file);

    // let mut archive = zip::ZipArchive::new(file).expect("Invalid archive");
    // println!("{:?}", archive);

    // let secret = do_loop(file_path.to_string())?;
    // let secret = do_loop(&mut archive)?;

    let sequence = "foo".to_string();

    let mut map = std::collections::HashMap::new();
    map.insert("sequence", sequence.trim());

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{}{}{}", SOLVE_URL, "?access_token=", access_token))
        .json(&map)
        .send()?;

    let j = response.text();

    println!("{:?}", j);

    Ok(())
}
