
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        active: true,
        username: String::from("realUserName"),
        email: String::from("realUserName@163.com"),
        sign_in_count: 1,
    };

    let mut mutUser = User {
        active: true,
        username: String::from("mutUserName"),
        email: String::from("mutUserName@163.com"),
        sign_in_count: 2,
    };

    mutUser.email = String::from("mutUserName1@163.com");
}
