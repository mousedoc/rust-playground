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

    let length: String = "          ";
    let length = length.len();
}

fn basic_constant() {
    const const_value: i32 = 123;

    println!("const_value is {}", const_value);

}