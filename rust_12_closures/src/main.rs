fn main() {
    // println!("Hello, world!");

    let _identity_closure = |x: i32| x;
    let incr_closure = |x: i32| { x + 1 };
    let _multi_line_closure = |x: i32| {
        println!("multi line closure...");
        x + 1
    };

    let constant_one_closure = || 1;

    let a = 41;
    println!("{} + 1 = {}", a, incr_closure(a));
    println!("using constant one closure: {}", constant_one_closure());

    // closure can capture variable

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("{} == {} ? {}", x, y, equal_to_x(y));
}
