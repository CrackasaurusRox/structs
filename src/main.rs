#[derive(Debug)]
struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
}

fn main() {
    let email = String::from("someone@something.com");
    let username = String::from("some_user");
    let user = build_user(email, username);
    println!("{:?}", user);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}