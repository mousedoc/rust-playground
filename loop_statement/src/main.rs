fn main() {
    loop_example();
    while_example();
    for_example();
    for_with_array_example();
    iter_enumerate_example();
}

fn loop_example() {
    let mut i = 0;

    loop {
        println!("loop {}", i);
        i += 1;

        if i == 5 {
            break;
        }
    }
}

fn while_example() {
    let mut count = 5;

    while count > 0 {
        println!("while {}", count);
        count -= 1;
    }
}

fn for_example() {
    for num in (1..4).rev() {
        println!("{}", num);
    }

    println!("finish");
}

fn for_with_array_example() {
    let elems = ["asdf", "qwer", "zxcv"];
    for elem in elems {
        println!("{}", elem);
    }
}

fn iter_enumerate_example() {
    let idx: usize = get_frist_space_index(&("hello world".to_owned()));
    println!("{}", idx);
}

fn get_frist_space_index(str: &String) -> usize {
    let bytes: &[u8] = str.as_bytes();

    // iter() return collection
    // enumerate() return (index, elements)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    0
}