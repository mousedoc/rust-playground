fn main() {
    basic_string_slices();
    string_slices_with_borrow();
    integer_slices();
}

fn basic_string_slices() {
    // this is String
    // Same with String::from("hello world")
    let hello_world: String = "hello world".to_owned();

    // this is string slices (&str)
    let slices_hello_world: &str = "hello world";

    println!("{} {}", hello_world, slices_hello_world);

    // start..end expression (exclude end)
    let first_word: &str = &hello_world[0..5];       
    println!("{}", first_word);

    // Same
    let first_word: &str = &hello_world[..5];
    println!("{}", first_word);

    // .. is entire
    let whole_sting: &str = &hello_world[..];
    println!("{}", whole_sting);
}

fn string_slices_with_borrow() {
    // no need to 'mut'
    let hello_world = "hello world".to_owned();
    let first_word = get_first_word_with_string(&hello_world);
    
    // cannot, cuz first_word borrow hello_world
    // clear() makes empty string ("")
    // hello_world.clear();

    println!("first word is {}", first_word);

    // deliver by &str
    let first_word: &str = get_first_world_with_str(&hello_world[..]);
    println!("first word is {}", first_word);

    let first_word: &str = get_first_world_with_str("bye bye");
    println!("first word is {}", first_word);
}

// return type '&str' means "string slices"
fn get_first_word_with_string(str: &String) -> &str {
    let bytes: &[u8] = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    &str[..]
}

fn get_first_world_with_str(str: &str) -> &str {
    let bytes: &[u8] = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    &str[..]
}

// &str = String slices
// &[i32] = i32 slices
fn integer_slices() {
    let int_elems: [i32; 5] = [1, 2, 3, 4, 5,];
    let int_slices: &[i32] = &int_elems[2..];

    for slice in int_slices {
        print!("{}, ", slice);
    }
}