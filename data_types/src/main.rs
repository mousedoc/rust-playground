fn main() {
    signed_integer_tyeps();
    unsigned_interger_types();
    floating_types();
    boolean_type();
    character_types();
    tuple_example();
    array_example();
}

fn signed_integer_tyeps() {
    let interger = 12;                                  // i32 - defualt is 32bit
    let integer_8: i8 = -12;        
    let integer_16: i16 = -120;     
    let integer_32: i32 = -1200;        
    let integer_64: i64 = -1200;        
    let integer_via_arch: isize = -12000;               // in x64 - 64bit, in x86 - 32bit

    println!("{}", integer_8);
    println!("{}", integer_16);
    println!("{}", integer_32);
    println!("{}", integer_64);
    println!("{}", integer_via_arch);
}


fn unsigned_interger_types() {
    let unsigned_integer_8: u8 = 120;
    let unsigned_integer_16: u16 = 1200;
    let unsigned_integer_32: u32 = 12000;
    let unsigned_integer_64: u64 = 120000;
    let unsigned_integer_via_arch: usize = 120000;      // in x64 - 64bit, in x86 - 32bit

    println!("{}", unsigned_integer_8);
    println!("{}", unsigned_integer_16);
    println!("{}", unsigned_integer_32);
    println!("{}", unsigned_integer_64);
    println!("{}", unsigned_integer_via_arch);
}

fn floating_types() {
    let float = 12.4;           // f64 - default is 64bit
    let float_32: f32 = 2.0;
    let float_64: f64 = 4.2;

    println!("{}", float);
    println!("{}", float_32);
    println!("{}", float_64);
}

fn boolean_type() {
    let mut flag: bool = false;
    flag = true;
    println!("{}", flag);
}

fn character_types() {
    let character: char = 'z';
    let characters = "zxcv";

    println!("{}", character);
    println!("{}", characters);
}

fn tuple_example() {
    let tuple = (500, "asdf", 1);
    let (x, y, z) = tuple;
    println!("{}, {}, {}", x, y, z);

    let tuple: (i32, f32, bool) = (512, 256.28, false);
    let first = tuple.0;
    let second = tuple.1;
    let third = tuple.2;
    println!("{}", first);
    println!("{}", second);
    println!("{}", third);
}

fn array_example() {
    let int_array = [1, 2, 3, 4, 5,];
    let str_array = ["a", "b", "c", "d",];      // &str is string slices *not String
    println!("{}", int_array[2]);
    println!("{}", str_array[3]);
}