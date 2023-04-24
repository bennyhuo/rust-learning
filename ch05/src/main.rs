use std::fmt::{Display, Formatter};

fn main() {
    let mut user1 = User {
        active: true,
        username: "bennyhuo".to_string(),
        email: "bennyhuo@kotliner.cn".to_string(),
        sign_in_count: 1,
    };

    user1.email = "bennyhuo@abc.com".to_string();

    let user2 = build_user("bennyhuo@kotliner.cn", "bennyhuo".to_string());
    println!("user2: {user2}");

    let user2 = User {
        email: "admin@bennyhuo.com".to_string(),
        ..user1
    };

    let black = Color(0, 0, 0);
    println!("{}", black.0);
}

struct AlwaysEqual;


struct Color(i32, i32, i32);

fn build_user(email: &str, username: String) -> User {
    User {
        email: email.to_string(),
        username,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("username: {}", self.username).as_str())
    }
}