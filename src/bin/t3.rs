use std::{sync::mpsc::channel, thread, time};

fn job(data: String) -> (String, usize) {
    thread::sleep(time::Duration::from_secs(1));
    let size = data.len();
    (data.clone(), size)
}

fn main() {
    let data_list = vec!["a".to_string(), "bla bla".to_string(), "cc".to_string()];

    let (tx, rx) = channel();

    for data in data_list {
        let tx = tx.clone();
        thread::spawn(move || tx.send(job(data)).unwrap());
    }
    drop(tx); // It's important!

    let res: Vec<_> = rx.iter().collect();
    println!("{:?}", res);
}
