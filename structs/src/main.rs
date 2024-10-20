struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // tuple structs
struct Point(i32, i32, i32);

struct AlwaysEqual; // this is a Unit-like struct

fn build_user(email: String, username: String) -> User {
    // this uses field init shorthand
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // entire instance must be mut or immutable
    let user1 = User {
        active: true,
        username: String::from("Max"),
        email: String::from("email@email.com"),
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("John"), String::from("smith"));
    let email = user1.email;
    println!("{email}");
    println!("{}", user2.email);

    // update syntax
    let user2_updated = User { // uses equal as it moves the data
        email: String::from("john@smith.com"),
        ..user2 // must come last,
    };

    println!("{}", user2_updated.email);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0); // use a . to access elements, e.g. _origin.0

    let subject = AlwaysEqual;
}
