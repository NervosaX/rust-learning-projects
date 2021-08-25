extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let random_num = thread_rng().gen_range(1..=100);

    loop {
        let mut buffer = String::new();
        let answer: i32;

        println!("Guess the number between 1 and 100!");

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Should have a value");

        match buffer.trim().parse::<i32>() {
            Ok(num) => {
                answer = num;
            }
            Err(_) => {
                println!("Expected a number");
                continue;
            }
        }

        match answer.cmp(&random_num) {
            Ordering::Greater => {
                println!("Number is lower");
            }
            Ordering::Less => {
                println!("Number is greater");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
