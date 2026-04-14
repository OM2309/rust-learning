fn main() {
    let counter = 0;
    let mut inc = || {
        counter += 1;
        println!("Counter: {}", counter);
    };

    inc();
    inc();
}
