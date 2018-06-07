extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the random name generator!");
    println!("What's your first name?");

    let mut first_name = String::new();

    io::stdin().read_line(&mut first_name)
        .expect("failed to read line");
    
    let mut last_name = String::new();

    io::stdin().read_line(&mut last_name)
        .expect("failed to read line");

    println!("Your name is {} {}", first_name, last_name);

    let vec = vec!["the man", "big-dawg", "cock-sucker"];

    let n_name = rand::thread_rng().choose(&vec);

    println!("Your new name is {} '{:?}' {}", first_name, n_name, last_name);
}