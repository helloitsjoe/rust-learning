use futures::future::select_all;
use std::fs;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// https://0xf.at/play/20
// Remember to cargo build --release!
pub async fn crack() {
    tokio_threads().await;
    // multi_thread();
}

fn multi_thread() {
    let file = fs::read_to_string("./wordlist.txt").expect("Could not read file");
    let file_words = file
        .trim()
        .split('\n')
        .map(|w| w.to_string())
        .collect::<Vec<String>>();

    let words = Arc::new(file_words);
    let done = Arc::new(Mutex::new(false));

    let mut handles = vec![];

    for i in 0..8 {
        let words = Arc::clone(&words);
        let done = Arc::clone(&done);
        let handle = thread::spawn(move || {
            let offset = words.len() / 8;
            let start = i * offset;
            let end = start + offset;

            for word1 in &words[start..end] {
                if *done.lock().unwrap() {
                    return;
                }

                println!("{:?}", word1);

                for word2 in &words[..] {
                    let digest = md5::compute(word1.to_string() + word2);
                    if format!("{:x}", digest) == "3df7c3057e540cbe9244561a2d4345f7" {
                        println!("{}{}", word1, word2);
                        let mut is_done = done.lock().unwrap();

                        *is_done = true;
                        return;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

async fn tokio_threads() {
    let file = fs::read_to_string("./wordlist.txt").expect("Could not read file");
    let file_words = file
        .trim()
        .split('\n')
        .map(|w| w.to_string())
        .collect::<Vec<String>>();

    let words = file_words;

    let mut futures: Vec<_> = vec![];

    for i in 0..8 {
        let cloned = words.clone();
        futures.push(tokio::spawn(async move { do_work(cloned, i).await }));
    }

    let result = select_all(futures).await;

    println!("DONE");
    println!("{:?}", result.0);
}

async fn do_work(words: Vec<String>, i: usize) -> Result<(), ()> {
    let offset = words.len() / 8;
    let start = i * offset;
    let end = start + offset;

    for word1 in &words[start..end] {
        println!("{:?}", word1);

        for word2 in &words[..] {
            let digest = md5::compute(word1.to_string() + word2);
            // TODO: Stop execution on all threads when password is found
            // if format!("{:x}", digest) == "3df7c3057e540cbe9244561a2d4345f7" {
            if format!("{:x}", digest) == "d8f8a1328020a6853495aec55990ccc9" {
                println!("{}{}", word1, word2);
                // let mut is_done = done.lock().unwrap();

                // *is_done = true;
                return Ok(());
            }
        }
    }

    Ok(())
}
