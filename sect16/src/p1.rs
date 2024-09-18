use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub fn run() {
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("I'm number {i} from a thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("I'm number {i} from the main thread");
            thread::sleep(Duration::from_millis(1));
        }
    }

    {
        let v = vec![1, 2, 3];

        // Rust infers that it can borrow v because of how it is used
        // However, since the main thread might free v before the thread does anything, rust won't
        // let this compile
        // Using move forces the thread to take ownership of the variables it uses
        let handle = thread::spawn(move || {
            println!("The vector: {v:?}");
        });

        handle.join().unwrap();
    }
}
