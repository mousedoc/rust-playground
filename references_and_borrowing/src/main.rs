fn main() {
    reference_example();
    mutable_reference_example();
    reference_scope_example();
}

fn reference_example() {
    let foo: String = String::from("foo");
    let len: usize = get_length(&foo);
    println!("{} {}", foo, len);

    let mut hello: String = String::from("hello");
    hello.push_str(" bye");
    let ref_hello: &String = &hello;
    let ref_hello_2: &String = &hello;            // can ref twice
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

fn get_length(str: &String) -> usize {
    str.len()
}

fn append_foo_get_length(str: &mut String) -> usize {
    str.push_str("foo");
    str.len()
}