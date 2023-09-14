enum Coin {
    Penny,      // 1
    Nickel,     // 5
    Dime,       // 10
    Quater,     // 25
}

fn main() {
    println!("Hello, world!");
}


fn value_in_cents(coin: Coin) => u8 [
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
]