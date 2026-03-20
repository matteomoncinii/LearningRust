struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        //username: username, // standard way
        username, // alternative way
        //email: email, // standard way
        email, //alterantive way
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername1"),
        email: String::from("someone1@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("someone1_1@example.com");

    // Instances with struct update syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("someone2@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Super syntethic way
    let user3 = User {
        email: String::from("someone3@example.com"),
        ..user2
    };

    let my_email = String::from("someone4@example.com");
    let my_username = String::from("someusername4");
    let user4 = build_user(my_email, my_username);

    println!("{}", { user1.email });
    println!("{}", { user2.email });
    println!("{}", { user3.email });
    println!("{}", { user4.email });
    // println!("{}", { user2.email }); // this won't work because it's been copied into user3

    // Struct
    let black = Color(1, 2, 3);
    let origin = Point(1, 1, 1);
    println!("Sum of black values: {}", black.0 + black.1 + black.2);
    println!("Sum of origin values: {}", origin.0 + origin.1 + origin.2);
}
