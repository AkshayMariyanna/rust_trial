fn main() {
    let imm_user = User {
        email: String::from("sm@e.com"),
        username: String::from("sme"),
        active: true,
        sign_in_count: 1,
    };

    let mut mut_user = User {
        email: String::from("sm@e.com"),
        username: String::from("sme"),
        active: true,
        sign_in_count: 1,
    };

    mut_user.email = String::from("sm93@e.com");

    let user2 = User {
        email: String::from("some@e.com"),
        username: String::from("some"),
        ..imm_user
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
