fn main() {
    // let mut counter = 0;
    // let mut inc = || {
    //     counter += 1;
    //     println!("Counter: {}", counter);
    // };

    // inc();
    // inc();

    let x = String::from("Hello");
    let consume_and_retunn_x = || x;
    let y = consume_and_retunn_x();
    println!("y: {}", y);
}
