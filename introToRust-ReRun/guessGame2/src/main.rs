
use std::{cmp::Ordering};
use std::io;
use fastrand::{self};

fn main() {
    println!("Guess the Letter! (a-z)");
    let alphabet = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let i = fastrand::usize(..alphabet.len());
    let rand = alphabet[i];
    let rand_index = alphabet.iter().position(|&r| r == rand).unwrap();
    println!("{}", rand);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: char = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        let index = alphabet.iter().position(|&r| r == guess).unwrap();
 
        match index.cmp(&rand_index) {
            Ordering::Less => println!("Letter is too small!!"),
            Ordering::Greater => println!("Letter is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}