fn main() {
    // println!("Hello, world!");

    let i = 11;
    match i {
        // Not accepted at all?
        // x @ 1..10 => println!("Between 1 and 10 (value is: {}", x),
        // Accepted in rust 2018 but not in Rust 2021
        // x @ 1...10 => println!("Between 1 and 10 (value is: {}", x),
        // Note: 1..=10 is inclusive range (10 is in the range)
        x @ 1..=10 => println!("Between 1 and 10: {}", x),
        11 | 12 => println!("11 or 12"),
        _ => println!("Nop! Not good!"),
    };
}
