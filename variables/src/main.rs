fn main() {
    // create a const - always immutable
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // specify mut to allow variable mutations
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;

    // 'shadowing' y allows you to define new var with same name
    let y = y + 1;

    // start new scope, y is shadowed in the scope, and then shadow is removed
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is back to: {y}");
}
