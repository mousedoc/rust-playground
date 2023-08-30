use std::io;

fn main() {
    println!("Guess Number Game");
    println!("Input your guess");

    // Input 
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).
        expect("Failed to read line");

    println!("Your guess: {}", guess);
}