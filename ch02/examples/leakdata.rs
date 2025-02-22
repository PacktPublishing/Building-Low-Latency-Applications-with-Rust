use std::thread;

fn create_static_data() {
    // how to create a static data
    let data1: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    let t1 = thread::spawn(move || {
        print!("th1) data= ");
        dbg!(data1)
    });
    let t2 = thread::spawn(move || {
        print!("th2) data= ");
        dbg!(data1)
    });

    t1.join().unwrap();
    t2.join().unwrap();
}


fn main() {
    create_static_data();
}
