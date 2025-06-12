#[derive(Debug)]
pub struct MyItem {
    pub id    : i32,
    // ...
}

mod crud {
    use crate::MyItem;
    // Version with the borrow checker error
    // pub fn replace(item: &mut MyItem, new_val : MyItem) -> MyItem {
    //     let prev = *item;
    //     *item = new_val;
    //     prev
    // }
    pub fn replace(item: &mut MyItem, new_val : MyItem) -> MyItem {
        std::mem::replace(item, new_val)
    }
}

mod references {
    use crate::MyItem;

    pub fn immutables() {
        let s = String::from("hello");
        let r1 = &s; // Immutable borrow
        let r2 = &s; // Another immutable borrow
        println!("{}, {}", r1, r2); // Both references are valid
    }

    pub fn mut_and_immut() {
        let mut s = String::from("hello");
        let r1 = &mut s; // Mutable borrow
        // let r2 = &s; // Error: cannot borrow `s` as immutable because it is already borrowed as mutable
        r1.push_str(", world");
        println!("{}", r1); // Valid
    }

    pub fn mut_and_immut2() {
        let mut item = MyItem { id: 1 };
        let p_item = &item;       // immutable reference
        item.id = 2;

        println!("Item = {:?}", item);
    }

    pub fn only_immut() {
        let item = MyItem { id: 1 };
        let p_item = &item;
        let one_id = item.id;

        println!("Item = {:?} with id {one_id}", p_item);
    }

    pub fn vec_and_ref() {
        let mut myvec: Vec<i32> = vec![0, 1, 2, 3];
        let first: &i32 = &myvec[0];
        // myvec.push(4);       // ERROR!!! push modifies the vector so it could invalidate all the immutable references
        println!("First element is {}", *first);
    }
}


fn pointers() {
    println!("Pointers...");
    let x = 42;        // x is i32
    let x_addr = &x;   // x_addr is a pointer

    println!("x={x}");
    println!("These are the same:");
    println!("xp={:p}",x_addr);
    println!("xp={:p}",&x);   // same as above
}

// Another version could use the immutable reference to a String
// fn borrow_string(y : &String) {
fn borrow_string(y : &str) {
  if y.len() > 0 {
    println!("y = {}", y); // Valid
  }
}

fn borrowing() {
    println!("Borrowing...");
    let x = String::from("hello"); // `x` owns the String
    borrow_string(x.as_ref());             // borrowing as a slice
    println!("x = {}, World", x);
}

fn move_string() {
    let x = String::from("hello"); // `x` owns the String
    {
        let y = x; // Ownership moves to `y`, and `x` is no longer valid
        println!("{}", y); // Valid
    } // `y' goes out of scope, String is dropped
    // println!("{}", x); // Error! `x` is no longer valid
}

fn move_string2() {
    let x = String::from("hello"); // `x` owns the String
    {
        let y = &x;
        println!("y = {}", y); // Valid
    } // `y' goes out of scope, String is dropped
    println!("x = {}", x);
}

fn crud() {
    println!("replacing...");
    let mut itm: MyItem = MyItem { id : 1 };
    println!("item: {:?}", itm);
    let new_itm: MyItem = MyItem { id : 2 };
    crud::replace(&mut itm, new_itm);
    println!("New item: {:?}", itm);
}

fn refs() {
    use references::*;

    immutables();
    mut_and_immut();
    mut_and_immut2();
    only_immut();
    vec_and_ref();
}

fn main() {
    move_string();
    move_string2();
    pointers();
    borrowing();
    crud();
    refs();
}
