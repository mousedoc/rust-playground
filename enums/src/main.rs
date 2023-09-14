// define enum

// Basic enum
enum State {
    Todo,               // similar struct Todo;
    InProgress,         // struct InProgress;
    Finish,
}

// Enum with string variant
#[derive(Debug)]
enum IpAddressType {
    V4(u8, u8, u8, u8), 
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                           // No type
    Move(i32, i32),                 // i32, i32 tuple type
    Write(String),                  // string type
    ChangeColor(u32, u32, u32),     // u32 tuple
}

// can implement enum
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

struct IpAddress {
    ipv4: IpAddressType,
    ipv6: IpAddressType,
}

fn main() {
    state_example();
    ip_address_example();
    message_example();
}

fn state_example() {
    let todo: State = State::Todo;
    let in_progress: State = State::InProgress;
    let finish: State = State::Finish;

    check_state(&todo);
    check_state(&in_progress);
    check_state(&finish);

    // Cannot
    // if todo == State::Todo {
    //     println!("todo");
    // }
}

fn check_state(state: &State) {
    match state {
        State::Todo => println!("todo"),
        State::InProgress => {
            println!("in-progress");
        },
        State::Finish => {
            println!("finish!");   
        },
    }
}

fn ip_address_example() {
    let loopback_ip_info: IpAddress = IpAddress {
        ipv4: IpAddressType::V4(127, 0, 0, 1),
        ipv6: IpAddressType::V6(":1".to_owned()),
    };

    println!("ipv4: {:?}", loopback_ip_info.ipv4);
    println!("ipv6: {:?}", loopback_ip_info.ipv6);
}

fn message_example() {
    let quit = Message::Quit;
    let change_color = Message::ChangeColor(1, 2, 3);

    quit.call();
    change_color.call();
}