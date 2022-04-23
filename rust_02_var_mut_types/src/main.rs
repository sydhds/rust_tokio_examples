fn main() {
    // println!("Hello, world!");

    let x = 12;
    println!("x = {}", x);

    // cannot do this - x is immutable
    //x = 33;

    let mut y = 11;
    println!("y = {}", y);
    // y is mutable so we can change it
    y -= 1;
    println!("y = {}", y);

    let _z = 10;
    // here we redefine z so this is valid
    let _z = 9;

    // this line will not compile, type annotations is required
    // from doc: https://doc.rust-lang.org/std/primitive.str.html#method.parse

    // let guess = "42".parse().expect("Not a number!");
    // with type annotation it works
    let _guess: u32 = "42".parse().expect("Not a number!");
    // or using the turbofish
    let _guess = "42".parse::<u32>().expect("Not a number!");
    // here using turbofish with type: unsigned 64 bits
    let _guess = "42".parse::<u64>().expect("Not a number!");

    // Note: parse() returns a Result, expect return the Ok value in Result
    // as an example, here is some declaration of Result

    let r1: Result<u32, &str> = Ok(3);
    let r2: Result<u32, &str> = Err("Do not give a foo!");
    println!("r1 is ok: {}, is error: {}", r1.is_ok(), r1.is_err());
    println!("r2 is ok: {}, is error: {}", r2.is_ok(), r2.is_err());

    // Tuple declaration
    let t1: (i32, f64, u8) = (500, 6.4, 1);
    // Note: tuples do not provide trait `fmt::Display` but only trait `fmt::Debug`
    //       so we need to use {:?}
    // See https://doc.rust-lang.org/std/primitive.tuple.html section `Trait implementations`
    println!("t1: {:?}", t1);
    println!("t1: {} - {} - {}", t1.0, t1.1, t1.2);

    // Array declaration
    let _a1 = [1, 2, 3, 4, 5];
}
