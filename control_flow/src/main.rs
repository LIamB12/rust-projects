fn main() {
    // if expressions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if conditions must result to a bool
    // rust does not attempt to convert other types to bool like python or js
    // IE we cannot do
    // let number = 0;
    // if number...
    // Instead we need to do
    // if number != 0...

    // Am example with else if

    let number = 6;

    if number % 4 == 0 {
        println!("Num is divisible by 4")
    } else if number % 3 == 0 {
        println!("Num is divisible by 3")
    } else if number % 2 == 0 {
        println!("Num is divisible by 2")
    } else {
        println!("Num is not divisible by 4, 3, or 2")
    }

    // If statements are an expression, so they return a value
    // that means we can use them like this to create a ternary
    let condition = true;
    let _number = if condition { 5 } else { 6 };

    // This also means that arms of if statemnt must have same return types
    // This is because ruse needs to know the type of each variable at compile time
    // Means we cannot do this:
    // let _x = if condition { "string" } else { 6 };

    // Loops
    // Rust provides 3 types of loops: loop, for, and while

    // loop creates an infinite loop
    // loops can alsp return a value
    // we can create a counter loop, and return a value using the break keyword,
    // followed by the return value

    let mut counter = 0;
    let value = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the loop returned {value}");

    // we can also return from a loop, but that exits the current function

    // We can aply labels to loops, to break out of parents in a nested loop

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // while loops
    let mut count = 3;
    while count > 0 {
        println!("{count}");
        count -= 1;
    }

    println!("LIFTOFF!");

    // for loops
    let a = [1, 2, 3, 4, 5];
    for elem in a {
        println!("The value is {elem}");
    }

    // for loop with a range
    for num in (1..4).rev() {
        println!("{num}");
    }

    println!("LIFTOFF!")
}
