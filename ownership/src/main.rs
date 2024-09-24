fn main() {
    // primitive data types like i32, bool, float, etc are stored on the stack
    // this is because their size is always known
    // more complex types, whose size may change must be stored on the heap

    // For Strings of unknown length, we use String type, which manages data
    // allocated on the heap
    // We can create one from a string literal like this
    let mut s = String::from("hello");

    // this type of string can be mutated
    s.push_str(", world!");

    // if we assign s to a string literal instead, we would not be able to mutate it - only reassign
    // This allows for fast operation on literals - they can be hardcoded into the binary at
    // compile time.

    println!("{s}");

    // Each value in rust has an owner
    // there can only be one owner at a time
    // when the owner goes out of scope, the value is dropped

    // For simple data types, like int, when copying data, we can make deep copy simce size is known
    let x = 5;
    let _y = x;
    // this binds the value 5 to x, and then makes a copy of the value 5. and assigns it to y

    // For heap data, the process is different
    // String for example, a group of data is stored on the stack - len, capacity, and a pointer to
    // the actual data on the heap
    let s2 = String::from("hello");
    let s3 = s2;
    // When we do this copy, the data on the stack (ptr, cap, len) is copied and bound to s3.
    // We do not copy the actual string on the heap
    // This is called a shallow copy in other languages
    // When we call drop after the scope, we look at each of theses strings, and attempt to clear
    // their data from the heap
    // This would cause a double free error, possibly corrupting other data.
    // To handle this when s3 is assigned to s2, rust no longer considers s2 valid
    // This means we can no longer use the variable s2, rust will throw an error for invalidatd ref
    // Because of this invalidation, this assignment is not called a shallow copy, but a move
    // Rust will never automatically make deep copies - copies are inexpensive

    // If we DO want to create a deep copy, we use clone()
    // This copies the heap data, and gives us a distinct pointer

    let _s4 = s3.clone();

    // We can place the Copy trait on types stored on the stack. These variables do not move when copied
    // If a type implements the Drop trait, it cannot implement Copy
    ownership_demo();
    ownership_demo2();
}

// after the scope ends, our variables are freed
// drop function is called

fn ownership_demo() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn ownership_demo2() {
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
