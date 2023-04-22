fn main() {
    println!("Hello, world!");
    println!("");
    (1..10).step_by(2).for_each(|i| {
        println!("{}", i);
    });
}
