struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let sahil = build_user(
        String::from("skariyania"),
        String::from("sahil.kariyania@gmail.com"),
    );

    // creating user with remaining values from exising user
    let soma = User {
        email: String::from("soma@gmail.com"),
        ..sahil
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
