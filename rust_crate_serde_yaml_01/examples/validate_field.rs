use serde::{Deserialize, Deserializer};

// A way to define some 'validator functions' for a particular field

fn de_above_2<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let v = u32::deserialize(deserializer)?;
    if v > 2 {
        Ok(v)
    } else {
        Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Unsigned(v as u64),
            &"a value above 2",
        ))
    }
}

fn de_non_empty_vec<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Vec::deserialize(deserializer)?;
    if !v.is_empty() {
        Ok(v)
    } else {
        Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Seq,
            &"a non empty vector'",
        ))
    }
}

#[derive(Debug, Deserialize)]
struct Struct {
    #[serde(deserialize_with = "de_above_2")]
    field_above_2: u32,
    #[serde(deserialize_with = "de_non_empty_vec")]
    vec_non_empty: Vec<u32>,
}

fn main() {
    println!("Hello world!");

    let s_yml0 = "field_above_2: 32\nvec_non_empty:\n  - 2\n  - 3\n";
    let s_yml1 = "field_above_2: 2\nvec_non_empty:\n  - 2\n  - 3\n";
    let s_yml2 = "field_above_2: 3\nvec_non_empty:\n";

    let s1: Struct = serde_yaml::from_str(&s_yml0).unwrap();
    println!("s1: {:?}", s1);

    // This will return an error -> value 2 is rejected
    let s2: Result<Struct, serde_yaml::Error> = serde_yaml::from_str(&s_yml1);
    println!("s2: {:?}", s2);

    // This will return an error -> empty vec is rejected
    let s3: Result<Struct, serde_yaml::Error> = serde_yaml::from_str(&s_yml2);
    println!("s3: {:?}", s3);
}
