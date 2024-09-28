use std::i32;

fn main() {
    // Rust is statically typed
    // When inference is not possible, we must add type annotation
    // IE the : u32 used here, to specify to parse into u32 int
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types:
    //     A scalar type represents a single value
    //     4 main scalar types - int, float, bool, char

    // ints - can be 8-128 bits, signed or unsigned
    // i32 = signed 32 bit int
    // u8 = unsigned 8 bit int
    // can also use isize and usize for arch defaults

    // If int overflow occurs in debug, rust will panic
    // If in release mode, rust performs 2's complement wrapping

    let x: i32 = 34;
    let y: u8 = 23;

    // float - either f32 or f64
    // generally use f64, as it is roughly the same speed on modern cpus, and more precise

    let z: f64 = 2.0;
    let a: f32 = 4.0;

    // Numeric operations

    // addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let quotient = 5.67 / 32.3;
    let truncated = -5 / 3; // -1, int division

    // remainder
    let remainder = 42 % 5;

    // Bool
    // bool is one byte in size

    let t = true;

    let f: bool = false;

    // char
    // must be specified with single quotes
    // 4 bytes in size
    // uses Unicode Scalar Value, supporting more than just ASCII

    let c = 'z';
    let d: char = 'Z';
    let heart_eyed_cat = '😻';

    // Compound Types
    // group multiple values into one type

    // Tuple
    // allows grouping of multiple types
    // types must be specified
    // tuples have fixed size

    // the variable tup is bound to the entire tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // we can use pattern matching to get values from a tuple
    // also called destructuring
    let (one, two, three) = tup;

    println!("The value of two is: {two}");

    // can also access a tuple element via index
    let tup2: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup2;
    let six_point_four = five_hundred.1;
    let uno = five_hundred.2;

    // The empty tuple () is called unit
    // its value and type are written ()
    // Expressions implicitly return unit if they don't return anything else

    // Arrays
    // Every element in an array must be the same type
    // Rust arrays have fixed length
    // Useful when you want data allocated on the stack rather than the heap

    let arr = [1, 2, 3, 4, 5];

    // annotate type and length
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];

    // init array to fill with value 3, and have length 5
    let filled_arr = [3; 5]; // [3,3,3,3,3]

    // Vectors
    // Like an array, but can grow and shrink
}
