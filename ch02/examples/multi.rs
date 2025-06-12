use std::thread;


fn multi_vec() {
    let mut v = vec![];
    v.push(0);

    thread::spawn(|| {  // E: closure may outlive the current function, but it borrows `v`,...
        v.push(1);
        println!("thread1");
    });

    thread::spawn(|| {  // E: cannot borrow `v` as mutable more than once...
        v.push(2);
        println!("thread2");
    });
    for el in v.iter() {  // E: cannot borrow `v` as immutable because it is also borrowed as mutable...
        println!("{el}");
    }
}

fn multi_vec_loop() {
    let mut v = vec![];
    v.push(0);

    for i in 0..2 {
        let thname = String::from("thread{i}");
        thread::spawn(|| {  // // E: closure may outlive the current function, but it borrows `v`...
            v.push(i);
            println!("{thname}");
        });
    }
    for el in v.iter() {   // E: cannot borrow `v` as immutable because it is also borrowed as mutable...
        println!("{el}");
    }
}

fn job_id(id: i32) -> thread::JoinHandle<()> {
    //thread::spawn(|| {    // E: closure may outlive the current function, but it borrows `id`,
    thread::spawn(move || {
        println!("job id {id}");
    })
}


fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    // drop(v);    // E: use of moved value: `v` value used here after move
    handle.join().unwrap();
}
// code from the Rust official docs: The Rust Programming Language Book
