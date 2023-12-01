struct User {
    active: bool,
    username: String,
    email: String,
    #[allow(dead_code)]
    sign_in_count: u64
}

#[allow(dead_code)]
pub fn run() {
    let user1 = User {
        email: String::from("cstague@iastate.edu"),
        username: String::from("Chris Tague"),
        active: true,
        sign_in_count: 1
    };

    let mut user2 = User {
        email: String::from("cstague@iastate.edu"),
        username: String::from("Chris Tague"),
        active: true,
        sign_in_count: 1
    };

    user2.active = false;

    let _user3 = User {
        username: String::from("Nathan"),
        ..user2 // oposite of JS destructure
    };

    // println!("User 2's email is {}", user2.email); // this doesn't work, because user3 now has ownership of the email string

    println!("{}'s email is {}", user1.username, user1.email);
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
