#[allow(clippy::useless_vec)]

fn main() {
    let _identity_closure = |x: i32| x;
    let incr_closure = |x: i32| x + 1;
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

    //
    let equal_to_x_ref = |z: &i32| *z == x;
    println!("{} == {} ? {}", x, y, equal_to_x_ref(&y));

    // increment count
    let mut count = 0;
    println!("count value: {}", count);
    // mutable closure as count is passded as &mut
    let mut inc_count = || count += 1;
    inc_count();
    println!("after increment: {}", count);

    // move (take ownership of captured variable)
    let v = vec![1, 2, 3];
    let take_vec = move || {
        println!("v sum: {}", v.iter().sum::<i32>());
    };
    take_vec();
    // Uncomment this line and compilation will fail as v is a vec (and it has non-copy semantics.)
    // println!("v: {}", v);
}
