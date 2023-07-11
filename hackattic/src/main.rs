mod hackattic;

use dotenv::dotenv;
use hackattic::touch_tone;

fn main() {
    dotenv().ok();
    let _ = touch_tone();
    // let _ = brute_force();
}
