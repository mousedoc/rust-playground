#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,

    // etc
}

enum Coin {
    Penny,               // 1
    Nickel,              // 5
    Dime,                // 10
    Quater(UsState),     // 25
}

fn main() {
    cent_example();
    option_example();
    full_fill_example();
}

fn cent_example() {
    let cent = get_value(Coin::Penny);
    println!("{}", cent);

    let cent = get_value(Coin::Nickel);
    println!("{}", cent);

    let cent = get_value(Coin::Dime);
    println!("{}", cent);

    let cent = get_value(Coin::Quater(UsState::Alabama));
    println!("{}", cent);

    let cent = get_value(Coin::Quater(UsState::Alaska));
    println!("{}", cent);
}

fn get_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("{:?} quater", state);
            25
        },
    }
}

fn option_example() {
    let five: Option<i32> = Some(5);
    let result: Option<i32> = plus_one(five);

    match result {
        None => println!("None"),
        Some(i) => println!("{}", i),
    }

    let four: Option<i32> = None;
    let three: Option<i32> = minus_one(four);

    match three {
        Some(i) => println!("{}", i),
        _ => println!("something wrong"),
    }
}

fn full_fill_example() {
    let num: u8 = 12;
    
    // cannot cuz has not all state of u8 
    // match num => {
    //     12 => println!("121212"),
    // } 

    // match should be full filled like below
    match num {
        12 => println!("12!"),
        other => {
            println!("otehr number {}", other);
        },

        // if you don't need 'other' value
        // can use '_'
    }

    // if you don't need 'other' value
    // can use '_'
    match num {
        12 => println!("12!"),
        _ => {
            println!("other number");
        },
    }

    match num {
        7 => (),        // this is nothing to do 
        other => {
            println!("{}", other);
        }
    }
}

// Define in rust prelude (defualti import things)
// enum Option<T> {
//     None,
//     Some(T),
// }
fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        // just None
        // not Option<i32>::None 
        None => None,
        Some(value) => {
            Some(value + 1)
        }
    }
}

fn minus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(value) => {
            Some(value - 1)
        },
        other => {
            println!("something wrong");
            None
        },
     }
}