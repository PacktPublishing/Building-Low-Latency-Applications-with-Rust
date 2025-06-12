pub fn f() -> i32 {
    let avar: i32 = 1;
    if avar > 0 {
        1
    } else {
        0
    }
}

fn main() {
    println!("Hello Chapter 02");
    println!("{}\n", f());
}
