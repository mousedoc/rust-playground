struct UserInfo {
    active: bool,
    user_name: String,
    email_address: String,
}

struct Color(i32, i32, i32);
struct Vector3(i32, i32, i32);

struct 

fn main() {
    basic_struct();
    update_struct_expression();
    struct_with_nameless_field();
    fieldless_struct();
}

fn basic_struct() {
    // create imutable a_user
    let a_user = UserInfo {
        active: true,
        user_name: "A".to_owned(),
        email_address: "A@gmail.com".to_owned(),
    };
    print_user_info(&a_user);

    // create mutable b_user
    let mut b_user: UserInfo = UserInfo {
        active: false,
        user_name: "BB".to_owned(),
        email_address: "moc.kooltuo@B".to_owned(),  
    };

    // modify b_user's email address
    b_user.email_address = "B@outlook.com".to_owned();
    print_user_info(&b_user);

    // create c_user with field initializer
    let mut c_user: UserInfo = build_user_info("CCC".to_owned(), 
                                               "C@icloud.com".to_owned());
    c_user.active = false;
    print_user_info(&c_user);
}

// filed initializer sample
fn build_user_info(user_name: String, email_address: String) -> UserInfo {
    let user = UserInfo {
        active: true,
        // should be name of parameter == name of struct field
        user_name,
        email_address,
    };

    user
}

fn update_struct_expression() {
    let a_user = build_user_info("Gave".to_owned(), 
                                           "Gave@gmail.com".to_owned());

    // Update expression
    let b_user = UserInfo {
        user_name: "Jave".to_owned(),

        // Now, canno use 'a_user' cuz this is '=' operator
        // and than update expression should be last 
        ..a_user
    };

    // cannot 
    // print_user_info(&a_user);
    print_user_info(&b_user);
}

fn struct_with_nameless_field() {
    let white: Color = Color(255,255,255);
    let origin = Vector3(0, 0, 0);

    println!("R is {}", white.0);
    println!("Z is {}", origin.2);

    // cannot, cuz different data type
    // let black: Color = Color {
    //     ..origin,
    // }
}

fn fieldless_struct() {

}

fn print_user_info(user: &UserInfo) {
    println!("{} {} {}", user.active, 
                         user.user_name, 
                         user.email_address);
}