mod hackattic;
mod oxfat;

use dotenv::dotenv;
// use hackattic::brute_force;
use oxfat::crack;

fn main() {
    dotenv().ok();
    crack();
    // let _ = brute_force();
}
