fn main() {
    example_function();
    one_param_function(123);
    two_param_function(256, 123.456);
    lambda_function();
    short_function();
}

fn example_function() {
    println!("bla bla");
}

fn one_param_function(num: i32) {
    println!("bla bla with {}", num);
}

fn two_param_function(a: i32, b: f64) {
    println!("a is {}, b is {}", a, b);
}

fn lambda_function() {
    let a = {
        let result = 5;
        result - 4
    };

    println!("a is {}", a);
}

fn return_five() -> i32 { 
    // same with return 5;
    5       
 }

 fn add(a: i32, b: i32) -> i32 {
    // same with return a + b;
    a + b
 }

fn short_function() {
    let num = return_five();
    println!("num is {}", num);
    println!("5 + 5 = {}", add(return_five(), return_five()));
}