fn main() {
    println!("Enter a number: ");
    let mut input = String::new();
    // .unwrap() panics the program
    std::io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    println!("Number: {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    println!("How old are you? ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let age: i32 = input.trim().parse().unwrap();
    println!("Age: {}", age);

    match age {
        0 => println!("I'm not born yet I guess"),
        // Match a range
        1..=12 => println!("I'm a child"),
        13..=19 => println!("I'm a teen"),
        // Match a single value
        20 => println!("I'm an adult"),
        // Match several values
        21 | 22 | 23 | 24 | 25 => println!("I'm a young adult"),
        // Match an inclusive range
        26..=100 => println!("I'm an adult"),
        // Match everything else
        _ => println!("I'm a senior"),
    }
}
