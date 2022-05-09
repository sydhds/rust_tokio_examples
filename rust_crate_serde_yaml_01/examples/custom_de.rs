use serde::{Deserialize, Deserializer};
use serde::de::Error;

// How to Deserialize a struct where an enum is driven by the field name

#[derive(Debug)]
struct Example {
    field: i32,
    an_enum: AnEnum,
}

/*

// Can also be achieved using the serde flatten macro

#[derive(Debug, Deserialize)]
struct Example {
    field: i32,
    #[serde(flatten)]
    an_enum: AnEnum,
}

 */

#[derive(Debug)]
enum AnEnum {
    A(i32),
    B(i32),
}

impl<'de> Deserialize<'de> for Example {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Debug, Deserialize)]
        struct Mapping {
            field: i32,
            #[serde(rename = "A")]
            a: Option<i32>,
            #[serde(rename = "B")]
            b: Option<i32>,
        }

        let Mapping { field, a, b } = Mapping::deserialize(deserializer)?;

        match (a, b) {
            (Some(_), Some(_)) => Err(D::Error::custom("multiple variant specified")),
            (Some(a), None) => {
                Ok(Example { field, an_enum: AnEnum::A(a) })
            }
            (None, Some(b)) => {
                Ok(Example { field, an_enum: AnEnum::B(b) })
            }
            (None, None) => {
                Err(D::Error::custom("No variants specified"))
            }
        }
    }
}


fn main() {

    let a = r#"{ "field": 42, "A": 42 }"#;
    let b = r#"{ "field": 42, "B": 110 }"#;

    let a: Result<Example, _> = serde_json::from_str(a);
    let b: Result<Example, _> = serde_json::from_str(b);

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    // Same but with yaml input

    let ya = "field: 42\nA: 42";
    let yb = "field: 42\nB: 110";

    let ya: Result<Example, _> = serde_yaml::from_str(ya);
    let yb: Result<Example, _> = serde_yaml::from_str(yb);

    println!("ya: {:?}", ya);
    println!("yb: {:?}", yb);
}
