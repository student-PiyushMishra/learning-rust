struct User {
    active: bool,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    sign_in_count: u32,
}

fn main() {
    let user: User = User{
        active: false,
        first_name: String::from("Piyush"),
        last_name: String::from(""),
        email: String::from("exmaple@xyz.com"),
        password: String::from("secret"),
        sign_in_count: 24
    };
    println!("{}",user.first_name);
}
