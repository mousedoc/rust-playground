struct UserInfo {
    active: bool,
    user_name: String,
    email_address: String,
}

fn main() {
    structs_example();
}

fn structs_example() {
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


fn print_user_info(user: &UserInfo) {
    println!("{} {} {}", user.active, 
                         user.user_name, 
                         user.email_address);
}