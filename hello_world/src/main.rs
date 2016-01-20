extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Secret {}",  secret);
    let mut count = 0;

    loop {

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("No1");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                count = count + 1;
                num
            },
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less      => println!("Too small"),
            Ordering::Greater   => println!("Too big"),
            Ordering::Equal     => {
                println!("You win! No tries: {}", count);
                break;
            }
        }
    }
}
