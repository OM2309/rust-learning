fn main() {
    let a = 5;
    let b = 6;
    println!("a: {}, b: {}", a, b);
    b = 6;
    println!("a: {}, b: {}", a, b);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("C: {}", THREE_HOURS_IN_SECONDS);
}
