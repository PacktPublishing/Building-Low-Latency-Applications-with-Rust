mod points {
    pub struct Point<'a> {
        pub x: &'a i32,
        pub y: &'a i32,
    }

    pub fn add_point<'a>(x: &'a i32, y: &'a i32) -> Point<'a> {
        Point {
            x,
            y,
        }
    }
}

fn moveit(str : String) {
    let _ = str; // move it here, just to consume it
} // the String goes out of scope here

// Simple example of how lifetimes works
 fn simple() {
    let x = String::from("hello"); // `x` owns the String
    {
        let y = "Hello, World";  // y lifetime starts here
        println!("{}", y);
    } // `y' goes out of scope, String is dropped
    // println!("{y}");    ERROR!!! y is already out of scope here!!
    let z = String::from("another string");  // z lifetime starts here
    moveit(z);   // z is moved here
    println!("{x}"); // x is still valid here
} // x goes out of scope here

fn relaxed() {
    let mut s: String = String::from("Hello, world");   // s owns the String

    let hello: &mut str = &mut s[..5];  // create a mut reference to a substring of s
    hello.make_ascii_uppercase();
    // .. 
    // suppose computation but NOT using hello
    //
    let ps: &str = &s;    // creating an immutable reference to the initial String
    println!("s = '{ps}'"); // s = 'HELLO, world'
    // Everything worked despite having mutable and immutable references in the same "scope"
} // all the references will be dropped here

fn points() {
    use points::*;

    let x = 1;
    let y = 2;
    let p : Point = add_point(&x, &y);
    println!("Point : x= {}, y= {}", p.x, p.y);
    
}

fn main() {
    simple();
    relaxed();
    points();
}
