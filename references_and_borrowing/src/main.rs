fn main() {
    reference_example();
    mutable_reference_example();
    reference_scope_example();
    dangling_ref_example();
    take_give_routine();
}

fn reference_example() {
    let foo: String = String::from("foo");
    let len: usize = get_length(&foo);
    println!("{} {}", foo, len);

    let mut hello: String = String::from("hello");
    hello.push_str(" bye");

    // can immutable ref twice
    let ref_hello: &String = &hello;
    let ref_hello_2: &String = &hello;            
    println!("{} {} {}", hello, ref_hello, ref_hello_2);
}

fn mutable_reference_example() {
    let mut big: String = String::from("big ");
    let len: usize = append_foo_get_length(&mut big);
    println!("{} {}", big, len);

    let mut hello: String = String::from("hello");

    // mutable refs
    {
        let ref_hello: &mut String = &mut hello; 
        ref_hello.push_str(" bye");    
        
        // cannot
        // println!("{}", hello);
        println!("{}", ref_hello);    
    }

    // imutable refs with mutable refs
    {
        let ref_hello: &String = &hello;
        let ref_hello_2: &String = &hello;
        // let mut_ref_hello: &mut String = &mut hello;        // cannot, cuz imutable ref already exist

        println!("{}", ref_hello);      // now, expired ref_hello
        println!("{}", ref_hello_2);    // now, expried ref_hello_2

        let mut_ref_hello: &mut String = &mut hello;        // available, cuz all imutable refs expired
        println!("{}", mut_ref_hello);
    }
}

fn reference_scope_example() {
    let mut str: String = String::from("some string");
    {
        let r1: &mut String = &mut str;
        r1.push_str(" some some");
        
        // now expired 'r'
    }  

    let r2: &mut String = &mut str;
    r2.push_str(" string string");
    println!("{}", r2);
}

fn dangling_ref_example() {
    let mut str: String = not_dangle();
    str.push_str(" good");
    println!("{}", str);
}

// cannot
// after return str refs than str expired -> compile error
// fn dangle() -> &String {
//     let str = String::from("dangle");
//     &str 
// should be below
fn not_dangle() -> String {
    let str: String = String::from("not dangle");
    str
}

fn get_length(str: &String) -> usize {
    str.len()
}

fn take_give_routine() {

    let hello = "hello".to_owned();
    let mut get_hello = take_give_string(hello);
    get_hello.push_str(" bye");

    println!("{}", get_hello);
}

fn append_foo_get_length(str: &mut String) -> usize {
    str.push_str("foo");
    str.len()
}

fn take_give_string(str: String) -> String {
    str
}
