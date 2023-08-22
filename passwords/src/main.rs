mod hackattic;
mod oxfat;

use dotenv::dotenv;
// use hackattic::brute_force;
use oxfat::crack;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();
    crack().await;
    // let _ = brute_force();
}
