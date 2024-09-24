struct User {
    active: bool,
    username: String, // Use owned string type soo each instance owns its data, and is always valid
    email: String,    // If we wanted to use &str, we'd need to add a lifetime specifier
    sign_in_count: i64,
}

// Tuple Structs allow us to give names to a tuple, without naming the values
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// We can also define unit-like structs, with similar behaviour to ()
// These are useful for adding traits to a type, without storing any data
struct AlwaysEqual;

fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // Create a mutable instance of User
    // The entire instance must be mutable - can't be only certain values
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Use dot notation to mutate/access values
    user1.email = String::from("anotheremail@example.com");
    user1.active = false;
    user1.username = String::from("sdkjlskdjfsd");
    user1.sign_in_count = 3;

    // We can use struct update syntax to create a new user with a few changed values
    // Since we move the String type of username into user2, we can no longer access it from user1
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1 // must come last
    };

    // Tuple Struct Creation
    // Each of theses are a different type (Color vs Point) even though the tuple values and types
    // match
    let origin = Point(0, 0, 0);
    let black = Color(0, 0, 0);
    let Point(x, y, z) = origin;
    let Color(r, g, b) = black;
    println!("Origin at {x} {y} {z}");
    println!("Black = rgb({r} {g} {b})");

    // Unit-like struct
    let subject = AlwaysEqual;
}
