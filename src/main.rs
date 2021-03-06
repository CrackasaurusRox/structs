#[derive(Debug)]
struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(f32, f32, f32);

fn main() {
    let email = String::from("someone@something.com");
    let username = String::from("some_user");
    
    let user1 = build_user(email, username);
    println!("{:?}", user1);

    let user2 = User {
        email: String::from("someoneelse@something.com"),
        username: String::from("some_other_user"),
        ..user1
    };
    println!("{:?}", user2);

    let black = Color(0,0,0);
    println!("{:?}", black);

    let origin = Point(0.,0.,0.);
    println!("{:?}", origin);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}