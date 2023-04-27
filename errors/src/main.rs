enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;

fn main() {
    println!("Hello, world!");
    // unrecoverable errors with panic
    println!("----------Unrecoverable Errors with Panic ----------");

    // get user input
    println!("Enter an integer");
    let mut input = String::new();
    println!("Input some text...");
    std::io::stdin().read_line(&mut input).unwrap();

    // unwrap panics if there is no integer input
    input.trim_end().parse::<i32>().unwrap();

    println!("----------Recoverable Errors with Result ----------");
    
}
