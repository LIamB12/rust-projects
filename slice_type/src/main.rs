fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

// A way of finding the first word of a string without slices
// finds the index of the end of the first word
// Causes a problem if the string is mutated
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// This functions returns the first word using a slice instead of an index
// type of slice is &str
// This stops the issue of the string being mutated and then using word, because we cannot
// use the returned immuatable borrow after a mutable borrow is used
fn sliced_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// &str generally better than &String because we can pass more values without losing functionality
// Can also make array slices with syntax:
fn _arr_demo() {
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
}
