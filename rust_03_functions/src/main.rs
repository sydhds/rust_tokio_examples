
fn main() {
    println!("Hello, world!");

    simple_function(3, 11);
    println!("{} + {} = {}", 3, 11, add(3, 11));
    println!("{} + {} = {}", 255, 250, add(255, 250));
    println!("{} + {} = {}", 4200, 4242, add_ret(4200, 4242));
}

fn simple_function(x: i32, y: i32) {
    println!("x: {}", x);
    println!("y: {}", y);
}

fn add(x: i32, y: i32) -> i32 {
    // return is not mandatory
    x + y
}

// This will not compile - need a return type
/*
fn add_no_rtype(x: i32, y: i32) {
    // return is not mandatory
    x + y
}
*/

fn add_ret(x: i32, y: i32) -> i32 {
    return x + y;
}
