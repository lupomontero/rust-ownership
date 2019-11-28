use std::sync::{Mutex, Arc};
use std::thread;

fn sum(counter: &Arc<Mutex<u64>>, from: u32, to: u32) -> std::thread::JoinHandle<()> {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        for i in from..to {
            let mut num = counter.lock().unwrap();
            *num += i as u64;
        }
    });

    handle
}

fn main() {
    let counter = Arc::new(Mutex::new(0_u64));
    let mut handles = vec![];

    handles.push(sum(&counter, 1, 50_000_000));
    handles.push(sum(&counter, 50_000_000, 100_000_001));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *counter.lock().unwrap());
}
