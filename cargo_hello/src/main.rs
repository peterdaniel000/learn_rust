extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("welcome to guessing game");


    loop{
        println!("your guess please");

        let mut guess =  String::new();

        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("the secret number is: {}", secret_number);


        io::stdin().read_line(&mut guess)
            .expect("failed to get input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input a valid number");
                continue;
            }
        };

        println!("your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
