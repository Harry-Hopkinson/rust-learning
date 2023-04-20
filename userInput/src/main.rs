use std::io;

// or you can use text_io
use text_io::scan;

fn main() {
    // std method
    let mut input = String::new();
    println!("Please input a number");
    
    input = match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => String::from(""),
    };
    print!("You entered: {}", input);

    // text_io method

    let mut input2 = String::new();
    println!("Please input another number");

    scan!("{}", input2);
    println!("You entered: {}", input2);
    
}
