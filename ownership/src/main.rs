fn main() {
    copy_example();
    move_example();
    clone_example();
    take_ownership_example();
    give_onwership_example();
}

fn copy_example() {
    // if type has a 'Copy' trait =operator will copy value
    // ex) bool, i32, u32, f64, Tuple of copyable types
    let x: i32=  5;
    let y: i32 = x;
    println!("x: {}", x);
    println!("y: {}", y);
}

fn move_example() {
    // Allocate "hello"(string) to first
    let first: String = String::from("hello");

    // Move ownership of first to second (not shallow/deep copy)
    // Now, first is invalid
    let second: String = first;
    println!("{}", second);

    // So not available using fisrt variable
    // println!("{}", first);

    // Move again to 'third'
    let mut third = second;
    third.push_str(" now third");
    println!("{}", third);

    // now cannot use 'second'
    // println!("{}", second);
}

fn clone_example() {
    // Deep copy
    let a: String = String::from("bye");
    let b: String = a.clone();
    println!("{} {}", a, b);
}

fn take_ownership_example() {
    let some_num: i32 = 512;
    print_num(some_num);

    // still available some_num
    println!("{}", some_num);   

    let some_str: String = String::from("hello? bye?");
    print_str(some_str);

    // no longer available
    // println!("{}", some_str); 
}

fn give_onwership_example() {
    // with new variable
    let hello: String = String::from("hello");
    let new_hello: String = print_str_and_return(hello);
    println!("yeah {}", new_hello);

    // with shadowing
    let bye: String = String::from("bye");
    let bye = print_str_and_return(bye);
    println!("yeah {}", bye);

    // with mut variable
    let mut foo = String::from("foo");
    foo = print_str_and_return(foo);
    println!("oh {}", foo);
    
    // with tuple
    let world: String = String::from("world");
    let world: (usize, String) = return_str_with_length_tuple(world);
    println!("{}", world.1);
}

fn print_str(str: String) {
    println!("{}", str);
}

fn print_num(num: i32) {
    println!("{}", num);
}

fn print_str_and_return(str: String) -> String {
    println!("{}", str);
    str
}

fn return_str_with_length_tuple(str: String) -> (usize, String) {
    // Not available str by first element
    // because ownership is returned and then len() is called.
    // (str, str.len())
    (str.len(), str)
}