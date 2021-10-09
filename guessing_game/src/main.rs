use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Guess The Number!\n");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please Enter Your Guess (1-100):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your Guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal =>  {
                println!("You Win!");
                break;
            }
        }
    }
}
