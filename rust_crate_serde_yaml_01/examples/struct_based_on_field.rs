use serde::{Deserialize, Serialize};

// How to deserialize to an enum based on a given tag
// From: https://serde.rs/enum-representations.html

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cfg")]
enum Config {
    FooConfig { field1: u32 },
    BarConfig { field1: u64, field2: String },
}

fn main() {
    // SE

    let foo = Config::FooConfig { field1: 32 };
    let bar = Config::BarConfig {
        field1: 78512u64,
        field2: "Let's go!".to_string(),
    };
    let l0 = vec![foo, bar];
    let s: String = serde_yaml::to_string(&l0).unwrap();
    println!("s: {}", s);

    // DE

    let s_yml0 = "- cfg: FooConfig\n  field1: 22";
    let s_yml1 = "- cfg: FooConfig\n  field1: 22\n- cfg: FooConfig\n  field1: 42";
    let s_yml2 =
        "- cfg: FooConfig\n  field1: 23\n- cfg: FooConfig\n  field1: 43\n- cfg: BarConfig\n  field1: 11\n  field2: yes";
    let s_yml3 =
        "- cfg: FooConfig\n  field1: 23\n- cfg: FooConfig\n  field1: 43\n- cfg: BarConfig2\n  field1: 11\n  field2: ye";

    let s0: Vec<Config> = serde_yaml::from_str(&s_yml0).unwrap();
    println!("s0: {:?}", s0);
    let s1: Vec<Config> = serde_yaml::from_str(&s_yml1).unwrap();
    println!("s1: {:?}", s1);
    let s2: Vec<Config> = serde_yaml::from_str(&s_yml2).unwrap();
    println!("s2: {:?}", s2);
    let s3: Result<Vec<Config>, serde_yaml::Error> = serde_yaml::from_str(&s_yml3);
    println!("s3: {:?}", s3);
}
