use std::sync::{Mutex, Arc};
use std::thread;

fn sum(counter: &Arc<Mutex<u64>>, from: u64, to: u64) -> std::thread::JoinHandle<()> {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut partial = 0;
        for i in from..to {
            partial += i;
        }
        let mut num = counter.lock().unwrap();
        *num += partial;
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
