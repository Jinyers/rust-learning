use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret number is: {}", secret_number);

    let mut guess: String;
    let mut guess_number: u32;

    loop {
        println!("Please input your guess.");

        guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        guess_number = guess
            .trim()
            .parse()
            .expect("Please type a number!");
    
        println!("You guessed: {}", guess_number);
    
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
