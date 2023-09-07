fn main() {
    variable();
    variable_shadowing();
    basic_constant();
}

fn variable() {
    let a = 5;
    println!("a is {}", a);

    // Not Available
    // a = 10;
    // println!("A is {}", a);

    let mut b = 10;
    println!("b is {}", b);

    // Available 
    b = 20;
    println!("b is {}", b);
}

fn variable_shadowing() {
    let num = 5;
    let num = num + 5;
    let num = num * 20;
    println!("num is {}", num);

    let length: String = "          ".to_string();
    let length: usize = length.len();

    println!("length is {}", length);
}

fn basic_constant() {
    const CONST_VALUE: i32 = 123;

    println!("const_value is {}", CONST_VALUE);
}