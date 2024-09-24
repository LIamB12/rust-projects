// adding this attribute allows us to derive the debug trait, so we can print rectangles
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block for Rectangle
// Self is an alias for the type Rectangle in its impl block
// methods must have a param named self of type Self
// In area, we borrow self, but we can also have them take ownership, or borrow mutably
// The functions in the impl block are called Associated Functions
// If an associated function takes self as their first param, then they are methods
// otherwise, they are similar to static functions - EX String::from()
// from is associated to the string type, but it cannot be called on a String instance
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width) && (self.height >= other.height)
    }

    fn square(side_length: u32) -> Self {
        Self {
            width: side_length,
            height: side_length,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // note that the area function expects a reference to self
    // Rust uses automatic referencing and dereferencing to allow the call to be placed directly on
    // our instance.
    // This means that we can just write rect.area() instead of (&rect).area()
    println!("The area of the rectangle is {}", rect.area());

    // {:?} allows us to print using the output format "Debug", a trait we added on line 1
    println!("{:?}", rect);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle::square(30);

    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect.can_hold(&rect4));
}
