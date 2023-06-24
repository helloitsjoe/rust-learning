use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek};
use std::sync::{Arc, Mutex};
use std::thread;
use zip::ZipArchive;

const BASE_URL: &str = "https://hackattic.com/challenges/brute_force_zip/problem";
const SOLVE_URL: &str = "https://hackattic.com/challenges/brute_force_zip/solve";

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

    let file_path = "./temp/file.zip";

    let zip_bytes = reqwest::blocking::get(zip_url)?.bytes()?;
    let _ = fs::write(file_path, zip_bytes);

    let file = fs::File::open(file_path).expect("File should exist");

    let mut archive = zip::ZipArchive::new(file).expect("Invalid archive");
    // println!("{:?}", archive);

    // let secret = do_loop(file_path.to_string())?;
    let secret = do_loop(&mut archive)?;

    let mut map = std::collections::HashMap::new();
    map.insert("secret", secret.trim());

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{}{}{}", SOLVE_URL, "?access_token=", access_token))
        .json(&map)
        .send()?;

    let j = response.text();

    println!("{:?}", j);

    Ok(())
}

fn try_decrypt<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    password: String,
) -> Result<String, reqwest::Error> {
    let res = archive.by_name_decrypt("secret.txt", password.as_bytes()); // decrypt first file in archive

    match res {
        Err(e) => panic!("Unexpected error {:?}", e),
        Ok(Err(_)) => {} // invalid password - continue
        Ok(Ok(mut a)) => {
            let mut b = String::new();
            a.read_to_string(&mut b);

            if !b.is_empty() {
                println!("{:?}", b);
                return Ok(b);
            }
        }
    };

    Ok(String::new())
}

// fn do_loop(file_path: String) -> Result<String, reqwest::Error> {
fn do_loop<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<String, reqwest::Error> {
    let range = "abcdefghijklmnopqrstuvwxyz0123456789";

    // ===
    // let done = Arc::new(Mutex::new(false));
    // let mut handles: Vec<std::thread::JoinHandle<Result<String, reqwest::Error>>> = vec![];

    // for r in 0..8 {
    //     let file_path = file_path.clone();
    //     let done = Arc::clone(&done);
    //     let handle = thread::spawn(move || {
    //         let file = fs::File::open(file_path).expect("File should exist");
    //         let mut archive = zip::ZipArchive::new(file).expect("Invalid archive");
    // ===
    // let offset = range.len() / 8;
    // let start = r * offset;
    // let end = start + offset;

    // for i in range[start..end].chars() {
    for i in range.chars() {
        println!("{:?}", i);
        // if *done.lock().unwrap() {
        //     return Ok("exiting".to_string());
        // }
        for j in range.chars() {
            for k in range.chars() {
                for l in range.chars() {
                    let password = format!("{}{}{}{}", i, j, k, l);
                    let cracked = try_decrypt(archive, password)?;
                    if !cracked.is_empty() {
                        // let mut is_done = done.lock().unwrap();
                        // *is_done = true;
                        return Ok(cracked);
                    }
                }
            }
        }
    }

    // for i in range[start..end].chars() {
    for i in range.chars() {
        println!("{:?}", i);
        // if *done.lock().unwrap() {
        //     return Ok("exiting".to_string());
        // }
        for j in range.chars() {
            for k in range.chars() {
                for l in range.chars() {
                    for m in range.chars() {
                        let password = format!("{}{}{}{}{}", i, j, k, l, m);
                        let cracked = try_decrypt(archive, password)?;
                        if !cracked.is_empty() {
                            // let mut is_done = done.lock().unwrap();
                            // *is_done = true;
                            return Ok(cracked);
                        }
                    }
                }
            }
        }
    }

    // for i in range[start..end].chars() {
    for i in range.chars() {
        println!("{:?}", i);
        // if *done.lock().unwrap() {
        //     return Ok("exiting".to_string());
        // }
        for j in range.chars() {
            for k in range.chars() {
                for l in range.chars() {
                    for m in range.chars() {
                        for n in range.chars() {
                            let password = format!("{}{}{}{}{}{}", i, j, k, l, m, n);
                            let cracked = try_decrypt(archive, password)?;
                            if !cracked.is_empty() {
                                // let mut is_done = done.lock().unwrap();
                                // *is_done = true;
                                return Ok(cracked);
                            }
                        }
                    }
                }
            }
        }
    }

    // ===
    // Ok("foo".to_string())
    // });

    // handles.push(handle);
    // }

    // for handle in handles {
    // handle.join().unwrap();
    // }

    // ===

    Ok("poo".to_string())
}
