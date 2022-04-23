fn main() {
    println!("Hello, world!");

    // if

    let answer = 42;
    if answer == 42 {
        println!("Thanks for all the fish!");
    } else if answer == 21 {
        println!("Thanks for almost all the fish!");
    } else {
        println!("Doctor!");
    }

    let x = 1;
    let y = if x == 1 { 3 } else { 5 };

    println!("x: {}, y: {}", x, y);

    // loop

    let mut i = 0;
    loop {
        println!("i: {}", i);
        i += 1;
        if i > 5 {
            break;
        }

    }

    // while
    println!("Now using a while loop =>");
    let mut i = 0;
    while i <= 5 {
        println!("i: {}", i);
        i += 1;
    }

    // for loop
    println!("Now using a for loop =>");
    for i in 1..4 {
        println!("i: {}", i);
    }

    let a = [10, 20, 30, 40, 50];
    for item in a.iter() {
        println!("a[x]: {}", item);
    }

    println!("Now with iterate (iter by ref)");  // edition 2021
    for item in a
        .iter()
        .enumerate()
    {
        let (i, value): (usize, &i32) = item;
        println!("a[{}]: {}", i, value);
    }

    println!("Now with iterate (iter by value)"); // edition 2021
    for item2 in a
        .into_iter()
        .enumerate()
    {
        let (i2, value2): (usize, i32) = item2;
        println!("a[{}]: {}", i2, value2);
    }

    // iter by value
    for item2 in a {
        let value: i32 = item2;
        println!("a[x]: {}", value);
    }
}
