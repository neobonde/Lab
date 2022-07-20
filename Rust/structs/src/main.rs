
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}


fn main() {

    let mut user1 = User {
        email: String::from("neobonde@gmail.com"),
        username: String::from("neobonde"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("spam.neobonde@gmail.com");

    let user2 = build_user(String::from("nkb@danalock.com"), String::from("nkb"));

    let user3 = User {
        email: String::from("123@gmail.com"),
        ..user1
    };


}

fn build_user(email: String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
