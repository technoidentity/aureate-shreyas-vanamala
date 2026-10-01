struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user = User {
        active: true,
        username: String::from("trainee01"),
        email: String::from("trainee@example.com"),
        sign_in_count: 1,
    };

    user.email = String::from("updated@example.com");

    println!(
        "User: {}, email: {}, active: {}, sign-ins: {}",
        user.username,
        user.email,
        user.active,
        user.sign_in_count
    );
}
