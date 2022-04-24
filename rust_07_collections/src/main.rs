use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {
    // println!("Hello, world!");

    // Vector

    let mut v1 = Vec::<i32>::new();

    println!("v1: {:?}", v1);
    v1.push(3);
    println!("v1: {:?}", v1);
    v1[0] = 5; // allowed because vec implements the Index and IndexMut traits
    println!("v1: {:?}", v1);

    let v2 = vec![10, 20, 30, 40 , 50 , 60];
    let v3 = vec![0.1; 10];
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);

    let item_last = v2.get(5);
    match item_last {
        Some(x) => println!("v2[5]: {}", x),
        None => println!("v2[5]: Out of range!"),
    }

    let item_last_2 = v2.get(6);
    match item_last_2 {
        Some(x) => println!("v2[6]: {}", x),
        None => { println!("v2[6]: Out of range!"); println!("yo!"); },
    }

    // Hashmap

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    println!("scores 1: {:?}", scores);

    let mut scores_2: HashMap<String, u32> = HashMap::new();  // explicit type
    scores_2.insert(String::from("Blue"), 10);
    scores_2.insert(String::from("Red"), 20);
    println!("scores 2: {:?}", scores_2);

    // Get items in HashMap, entry return result
    let e1 = scores.entry(String::from("Blue"));
    // Note: use 'ref x' to get a reference instead of borrowing value
    //       otherwise we cannot do a second match e1 { ... }
    match e1 {
        Occupied(ref x) => println!("Blue: {:?}", x),
        Vacant(ref _x) => println!("'Blue' not in hash map"),
    }
    // second match with direct value retrieval
    match e1 {
        Occupied(x) => { let v: &i32 = x.get(); println!("Blue: {}", v); },
        Vacant(x) => println!("'{}' not in hash map", x.key()),
    }

    scores.entry(String::from("Green")).or_insert(50);
    println!("scores 1: {:?}", scores);

    println!("Iter over hash map:");
    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }
    println!("scores: {:?}", scores);
}
