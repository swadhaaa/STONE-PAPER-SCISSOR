use std::io;
fn main() {
    println!("The number is even");
    let mut num = String::new();
    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut num) // actually read the line
        .expect("Failed to read line"); // which can fail, however
    let x: i32 = num
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer");
    if x % 2 == 0 {
        println!("The number is even")
    } else if x % 2 != 0 {
        println!("The number is odd")
    }
}
