// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// I AM NOT DONE

fn main() {
    // let a = {
    //     let mut foo = Vec::new();
    //     for num in 1..101 {
    //         foo.push(num)
    //     }
    //     foo
    // };

    let a = ["Hello"; 101];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
