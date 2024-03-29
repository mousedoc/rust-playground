extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn variable_example() {
    let a = 10;         // immutable
    let mut b = 20;     // mutable
    println!("Before, a was {} and b was {}", a, b);

    // a *= 10; not available
    b *= 10;

    println!("Now, a is {} and b is {}", a, b);
}

fn read_line_example() {
    println!("Input Example");

    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);

    // if result is 'Err' expect method print msg and stop program
    result.expect("something wrong");                   

    // r : raw string 
    // # - # : can conatin "
    println!(r#"Input is "{}""#, input);
}

fn random_example() {

    // Should be mutable
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2 = rng.gen::<u16>();
    
    // data type range
    println!("Rnadom u8: {}", n1);
    println!("Rnadom u16: {}", n2);
    println!("Rnadom u32: {}", rng.gen::<u32>());
    println!("Rnadom i32: {}", rng.gen::<i32>());
    println!("Rnadom f32: {}", rng.gen::<f32>());
    println!("Rnadom f64: {}", rng.gen::<f64>());

    // specific range
    println!("Random 0~10 (exclude 10): {}", rng.gen_range(0..10));
    println!("Random 0f~10f (exclude 10f): {}", rng.gen_range(0.0..10.0));        
}

fn compare_example() {

    let left = 10;
    let right = 5;

    match left.cmp(&right) {
        Ordering::Equal => println!("Equal"),
        Ordering::Greater => println!("Greater"),
        Ordering::Less => println!("Less"),
    }
}

fn main() {
    // variable_example();
    // random_example();
    // read_line_example();
    // compare_example();

    // 0~9
    let answer = rand::thread_rng().gen_range(0..10);

    println!("Guess Number Game");
    println!("Input your guess (0 ~ 9)");

    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).
                    expect("Failed to read line");

        let integer_input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input");
                continue;
            },
        };

        match integer_input.cmp(&answer) {
            Ordering::Greater => println!("More down"),
            Ordering::Less => println!("More up"),
            Ordering::Equal => {
                println!("Correct!");
                break;      // break loop
            }
        }
    }   
}

