struct User {
    active: bool,
    username: String,
    email: String,
}

fn create_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
    }
}

fn remove_user(email: String, username: String) -> User {
    User {
        active: false,
        username: username,
        email: email,
    }
}

pub fn users() {
    let mut sign_in_count = 0;
    let mut user1 = create_user(
        "someone@example.com".to_string(),
        "someusername123".to_string(),
    );
    sign_in_count += 1;

    user1.email = String::from("anotheremail@example.com");
    println!("User 1: {}", user1.email);

    let user2 = create_user(
        "some2@example.com".to_string(),
        "someusername2".to_string(),
    );
    sign_in_count += 1;

    println!("User 2: {}", user2.email);
    println!("Removing user 1...");
    remove_user("someone@example.com".to_string(), "someusername123".to_string());
    sign_in_count -= 1;
    println!("Sign in count: {}", sign_in_count)
}
