fn main() {
    let s1 = String::from("hello");

    // references allow a function to refer to a value without taking ownership
    let len = calculate_length(&s1);

    // len is still a valid reference because we passed a ref to the function
    // We did not move len to the function argument
    println!("Str len is {len}");

    // allow functions to change data while borrowing with a mut ref
    let mut s2 = String::from("hello");

    change(&mut s2);

    println!("String is now: {s2}");

    // Restriction with mutable refs:
    // You cannot create a mutable ref if any other mutable ref to the data exists
    // This blocks data races, when the two try to access the data at the same time
    // You also cannot create mut ref if immutable refs exist and are still used
    // this is because the immutable refs don't expect the data to change

    // Dangling Pointers
    // Rust doesn't allow creation of dangling pointers
    // if a variable goes out of scope before a ref to it does, it will not compile
    // If we want to return a String created in a function, we have to move
    // the string to the caller, not return a ref
    // This gives the caller ownership of the data
}

fn calculate_length(s: &String) -> usize {
    // s is a ref to a String
    s.len() //implicit return

    // Because we're borrowing the string, we cannot mutate it
    // Ex. s.push_str(", world!"); would throw an error, bc refs are immuatable by default
    // If we need to mutate it, we can specify that the ref is mutable
    // we do this in the next function
} // ref is dropped, but not the string that it refers to

fn change(s: &mut String) {
    s.push_str(", world!");
}
