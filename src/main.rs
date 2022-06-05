use std::io;
use std::cmp::Ordering;
use colored::*;
use rand::Rng;

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("The secret_number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => println!("You guessed:{guess}"),
            Err(_) => continue
        }

        let guess = match guess.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please type a Number!".on_bright_yellow());
                continue
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too smail!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "Yot win!!!".green());
                break
            },
        }
    }
}
