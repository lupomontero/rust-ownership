use std::thread;

fn main() {
    let mut a: u64 = 0;

    let one = thread::spawn(move || {
        for i in 1..50_000_000 {
            a = a + i;
        }
    });

    let two = thread::spawn(move || {
        for i in 50_000_000..=100_000_000 {
            a = a + i;
        }
    });

    one.join().unwrap();
    two.join().unwrap();

    println!("{:?}", a);
}
