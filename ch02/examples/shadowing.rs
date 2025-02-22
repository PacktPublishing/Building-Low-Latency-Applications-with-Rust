fn simple() {
    let x: i32 = 3;
    println!("simple x = {}", x);

    // Allowed. Shadowing and changing type
    let x:&str = "Rust is awesome";
    println!("{x}");
    
    let h: &str = "hello";
    println!("{h}");

    // Allowed. Changing type and even using the previous value
    let h: usize = h.len();
    println!("hello has {} characters", h);
}

// checking shadowing with different scopes
fn with_scope() {
    let x = 1i32;
    {
        let x = x + 10;
        println!("x inner scope = {}",x);
    }
    println!("x = {}",x);
}

fn main () {
    simple();
    with_scope();
}
