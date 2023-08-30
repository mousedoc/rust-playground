extern crate rand;

use std::io;
use rand::Rng;

fn let_example() {
    let a = 10;         // immutable
    let mut b = 20;     // mutable
    println!("Before, a was {} and b was {}", a, b);

    // a *= 10; not available
    b *= 10;

    println!("Now, a is {} and b is {}", a, b);
}

fn read_line_example() {
    println!("Input Example");

    let mut input = String::new();asdf
    let result = io::stdin().read_line(&mut input);

    // if result is 'Err' expect method print msg and stop program
    result.expect("something wrong");                   

    // r : raw string 
    // # - # : can conatin "
    println!(r#"Input is "{}""#, input);
}

fn main() {

    let_example();
    read_line_example();

    // 0~9
    let answer = rand::thread_rng().gen_range(0, 10);

    println!("Guess Number Game");
    println!("Input your guess");

    // Input 
    let mut input = String::new();
    io::stdin().read_line(&mut input).
        expect("Failed to read line");
    
    println!("Your input: {}", input);

}

