use std::{sync::mpsc::channel, thread, time};

fn job(num: i32) -> (i32, i32) {
    thread::sleep(time::Duration::from_secs(1));
    (num, num * num)
}

fn main() {
    let (tx, rx) = channel();

    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(job(i)).unwrap());
    }
    drop(tx); // It's important!

    let res: Vec<_> = rx.iter().collect();
    println!("{:?}", res);
}
