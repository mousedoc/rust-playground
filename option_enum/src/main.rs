
// Define in rust prelude (defualti import things)
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    option_example();
}

fn option_example() {
    let number: i8 = 5;
    let some_number: Option<i8> = Some(5);
    let some_char: Option<char> = Some('c');

    // cannot cuz, i8 - Option<i8> is difference type
    // let a = number + some_number;
    // println!("{}", a);
}
