use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut correct_guess = false;
    let rand_number = rand::thread_rng().gen_range(1..=100);
    println!("The random number is {}", rand_number);

    while correct_guess == false {
        println!("Guess the number!");
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().parse::<u32>().expect("Please type a number!");
        println!("You guessed: {guess}");

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => correct_guess = true,
        }
    }

    println!("You have guessed the number!");
}
