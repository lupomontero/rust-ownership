fn main() {
    let mut a: u64 = 0;

    for i in 1..=100_000_000 {
        a = a + i;
    }

    println!("{:?}", a);
}
