// use std::env::{args, Args};

fn main() {
    // Data type can be inferred. Rust defaults to the i32 type
    // isize, usize => 8, 16, 32, 64, 128
    // isize => signed: can be pos or neg; store -[2^(n-1)] to 2^(n-1) - 1
    // -2^(8-1) to 2^(8-1) - 1 => -128 to 127
    // usize => unsigned: 0 => 2^(n-1) => 2(8-1) 0 to 128
    let mut x: i32 = 1;
    println!("The value of x is: {}", x);

    x = 2;
    println!("The value of x is : {}", x);

    // booleans are 1 byte in size
    let y: bool = true;
    println!("The value of x is : {}", y);

    // Floating point: f32, f64
    // default type is f64

    // characters - represent letters
    // specified using the char keyword
    // use single quotes
    // four bytes in size
    let letter: char = 'a';
    println!("{}", letter);

    let phrase: &str = "hello world";
    println!("{}", phrase);
}
