use std::{thread, time};

fn job(num: i32) -> i32 {
    println!("job{} started", num);
    thread::sleep(time::Duration::from_secs(2));
    println!("job{} finished", num);
    num * num
}

fn main() {
    for i in 0..5 {
        thread::spawn(move || job(i));
    }

    thread::sleep(time::Duration::from_secs(3));
}
