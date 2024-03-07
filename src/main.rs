//to input from console
use std::io;
//for randomization
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    //this is secret number you are supposed to guess it ranges form 1 to 100 inclusively
    let secret_guess=rand::thread_rng().gen_range(1..=100);
    //displaying secret number
    println!("{secret_guess}");
    println!("please enter your guuess");
    //empty string for user guess
    let mut guess = String::new();

    //function that takes input
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("{guess}");
}
