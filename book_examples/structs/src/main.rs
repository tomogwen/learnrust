// rustbook - structs

// struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct tuples (types but no fields)
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs (nothing!)
struct AlwaysEqual;


fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User { 
        email: String::from("user@domain.com"),
        username: String::from("bloke"),
        active: true,
        sign_in_count: 1,
    };

    println!("username is {}", user1.username);
    user1.username = String::from("new bloke");
    println!("username is {}", user1.username);

    let user2: User = build_user(String::from("bob"), String::from("bob@smith.com"));
    println!("{} has logged in {} times", user2.username, user2.sign_in_count);

    // let user3 = User {
    //     email: user1.email,
    //     username: user1.username,
    //     active: user1.active,
    //     sign_in_count, user1.sign_in_count,
    // };
    // instead:
    let user3 = User {
        email: String::from("user3@users.com"),
        ..user1
    };

    println!("{}", user3.sign_in_count);

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
}
