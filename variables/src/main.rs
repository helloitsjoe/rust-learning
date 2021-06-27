use std::env;

mod fibo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args[1].parse().expect("Input must be a number.");

    // let f = 60;
    // let c = -75;

    // println!("{} fahrenheit is {} celsius", f, to_celsius(f));
    // println!("{} celsius is {} fahrenheit", c, to_fahrenheit(c));

    fibo::main(num);
}

// fn to_celsius(f: i32) -> i32 {
//     f * 5 / 9
// }

// fn to_fahrenheit(c: i32) -> i32 {
//     c * 9 / 5
// }
