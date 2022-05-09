use serde::{Serialize, Deserialize};

// How to deserialize to an enum based on a given key
// examples:
// cfg:
//   kind: foo
// --> Base on kind value, we want an CfgFoo or CfgBar

#[derive(Debug, Serialize, Deserialize)]
// #[derive(Debug)]
pub struct FooConfig {
    kind: String,
    field1: u32,
}

#[derive(Debug, Serialize, Deserialize)]
// #[derive(Debug)]
pub struct BarConfig {
    kind: String,
    field1: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Config {
    #[serde(deserialize_with = "my_cfg_fn")]
    #[serde(rename(serialize = "cfg", deserialize = "cfg"))]
    Foo(FooConfig),
    #[serde(deserialize_with = "my_cfg_fn")]
    #[serde(rename(serialize = "cfg", deserialize = "cfg"))]
    Bar(BarConfig),
}

fn my_cfg_fn<'de, T, D>(Deserialize) {

}


mod my_cfg {

    use super::{Config, FooConfig, BarConfig};
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(config: &Config, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str("jdslfjslkf")
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Config, D::Error>
        where D: Deserializer<'de>
    {
        Ok(
            Config::Bar(
                BarConfig { kind: "Bar".to_string(), field1: 180546895213 }
            )
        )
    }

}


fn main() {
    println!("Hello world!");

    // SE

    let foo = FooConfig { kind: "Foo".to_string(), field1: 32 };
    let bar = BarConfig { kind: "Bar".to_string(), field1: 78512 as u64 };
    let l0 = vec![Config::Foo(foo), Config::Bar(bar)];
    /*
    let s: String = serde_yaml::to_string(&l0).unwrap();
    println!("s: {}", s);
    */

    // DE

    let s_yml0 = "- cfg:\n    kind: foo\n    field1: 11\n- cfg:\n    kind: bar\n    field1: 465845321";
    let s_yml1 = "- cfg:\n  kind: foo\n- cfg:\n  kind: baz\n"; // baz is not valid

    let s0: Vec<Config> = serde_yaml::from_str(&s_yml0).unwrap();
    println!("s0: {:?}", s0);


}