extern crate rand;

use rand::Rng;

fn main() {
    if_else_example();
    let_with_if_statement();
}

fn if_else_example() {
    // 0~10 (exclude 10)
    let num = rand::thread_rng().gen_range(0.. 10);

    if num < 5 {
        println!("num is less than five");
    }
    else if num == 5 {
        println!("num is five");
    }
    else {
        println!("num is more than five");
    }
}

fn let_with_if_statement() {
    let num = rand::thread_rng().gen_range(0.. 10);
    let result = {
        if num > 5 {
            true
        }
        else {
            false
        }
    };

    // same expression
    let result = if num > 5 {
        true
    }
    else 
        false
    }

    println!("result is {}", result);
}