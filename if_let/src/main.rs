fn main() {
    type_suffix();
    if_let();
}

fn type_suffix() {
    let five = 5i32;
    let float = 12.23f64;

    println!("{}", five);
    println!("{}", float);
}

fn if_let() {
    // think let literally 'let~'
    let some_num: Option<i32> = Some(3i32);
    if let Some(num) = some_num {
        println!("{}", num)
    }

    // same expression with 'match'
    match some_num {
        Some(num) => {
            println!("{}", num);
        },
        _ => (),
    }
}