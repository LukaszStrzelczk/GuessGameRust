<<<<<<< HEAD


fn main() {
    println!("Hello, world!");
    
=======
use std::io;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("{guess}");
>>>>>>> 0cf40bbc27f41d1e5bec12cf9879c6e588e2f219
}
