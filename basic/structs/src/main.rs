struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    basic_struct();
    build_struct();
    build_two_structs();
    short_struct();
}

fn basic_struct() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    print_user_struct(user1);
}

fn build_struct() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    user1.email = String::from("anotheremail@example.com");

    print_user_struct(user1);
}

fn build_two_structs() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    print_user_struct(user1);
    print_user_struct(user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn short_struct() {
    let black = Color(11, 22, 33);
    let origin = Point(44, 55, 66);
    println!("{}", black.1);
    println!("{}", origin.2);
}

fn print_user_struct(user: User) {
    println!("[{};{};{};{}]", user.username, user.email, user.active, user.sign_in_count);
}