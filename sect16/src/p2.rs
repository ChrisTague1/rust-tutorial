use std::{sync::mpsc, thread, time::Duration};

#[allow(dead_code)]
pub fn run() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("there"),
            String::from("young"),
            String::from("man"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("fellow"),
            String::from("old"),
            String::from("woman"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
