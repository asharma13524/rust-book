struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someuser"),
        email: String::from("some_email"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("hello"),
        ..user1
    };

    user1.email = String::from("hello");
    let black = Color(0, 0, 0);

    let subject = AlwaysEqual;

}