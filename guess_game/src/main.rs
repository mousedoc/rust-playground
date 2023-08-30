use std::io;

fn main() {

    let a: i32 = 10;        // immutable
    let mut b: i32 = 20;    // mutable 

    println!("Before, a was {} and b was {}", a, b);

    // a = 150;             // not avilable
    b = 250;

    println!("Now, a is {} b is {}", a, b);

    println!("Guess Number Game");
    println!("Input your guess");

    // Input 
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).
        expect("Failed to read line");

    println!("Your guess: {}", guess);

    
    println!("Input Example");
    let result: Result<usize, io::Error> =io::stdin().read_line(&mut guess);
    result.expect("something wrong");

}