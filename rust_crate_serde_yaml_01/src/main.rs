use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

// type Test1Key = HashMap<String, Point>;

#[derive(Debug, Serialize, Deserialize)]
struct Test1 {
    points: HashMap<String, Point>,
}

fn main() -> Result<(), serde_yaml::Error> {
    // example 1

    let point = Point { x: 1.0, y: 2.0 };

    // Serialize point (type Point) to a yaml string
    let s: String = serde_yaml::to_string(&point)?;

    assert_eq!(s, "---\nx: 1.0\ny: 2.0\n");

    // Now de serialize our string to a point (type Point)
    let deserialized_point: Point = serde_yaml::from_str(&s)?;

    assert_eq!(point, deserialized_point);

    // example 2 - 0

    let pt_bob = Point { x: 1.0, y: 2.0 };
    let pt_alice = Point { x: 1.5, y: 99.27 };
    let h1 = HashMap::from([("Bob", pt_bob), ("Alice", pt_alice)]);
    let h0 = HashMap::from([("points", h1)]);

    let s: String = serde_yaml::to_string(&h0)?;
    println!("s: {}", s);

    // example 2 - 1

    let pt_bob = Point { x: 1.0, y: 2.0 };
    let pt_alice = Point { x: 1.5, y: 99.27 };
    let h1: HashMap<String, Point> =
        HashMap::from([("Bob".to_string(), pt_bob), ("Alice".to_string(), pt_alice)]);
    let h0 = Test1 { points: h1 };

    let s: String = serde_yaml::to_string(&h0)?;
    println!("s: {}", s);

    //

    let f = std::fs::File::open("test1.yml").unwrap(); // too lazy to handle this error ;)
    let d: Test1 = serde_yaml::from_reader(f)?;

    println!("Read yaml string: {:?}", d);

    Ok(())
}
