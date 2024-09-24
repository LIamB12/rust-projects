fn main() {
    println!("Hello, world!");
    another_function(23);
    print_label_measurement(43, 'm');

    // Statements and Expressions
    // Statements perform an action and do not return a value
    let _x = 5; // this is a statement
                // Since statements do not return a value, we cannot bind variables to them
                // EX let y = (let x = 5); does not work, since let x = 5 is a statement

    // Expressiosns evaluate to a result
    // 5 + 6 is an expression resulting to 11
    // 5 is a statement resulting to 5
    // function and macro calls are expressions - they return a value - can be unit ()
    // A new scope block is also an expression
    // By not including a ; after x + 1, it becomes an expression
    // functions and scope blocks implicitly return the last expression evauated
    // you can also return early with the return keyword
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}"); // 4
    let x = another_function(4);
    println!("{x}");
}

fn another_function(x: i32) -> i32 {
    println!("The value of x is: {x}");
    4 + 5
}

fn print_label_measurement(value: i32, unit_label: char) {
    let x = 5;
    println!("The measurement is: {x} {value}{unit_label}")
}
