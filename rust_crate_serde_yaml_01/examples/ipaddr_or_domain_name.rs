use std::collections::HashMap;
use std::convert::TryFrom;
use std::net::{IpAddr, AddrParseError};

use serde::{Serialize, Deserialize};

// Howto to serialize / deserialize to an enum with the same field name

/*
#[derive(Debug, Serialize, Deserialize)]
struct Hostname0 {
    host: Option<String>,
    ipaddr: Option<std::net::IpAddr>,
}
*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from="HashMap<String, String>")]
enum Hostname {
    #[serde(rename(serialize = "hostname", deserialize = "hostname"))]
    IpAddr(Option<IpAddr>),
    #[serde(rename(serialize = "hostname", deserialize = "hostname"))]
    Host(Option<String>),
}

impl TryFrom<HashMap<String, String>> for Hostname {

    type Error = &'static str;

    fn try_from(h: HashMap<String, String>) -> Result<Self, Self::Error> {

        if let Some((_k, v)) = h.get_key_value("hostname") {

            let ip_addr_: Result<IpAddr, AddrParseError> = (*v).parse();
            return match ip_addr_ {
                Ok(ip_addr) => { Ok(Hostname::IpAddr(Some(ip_addr))) },
                Err(_) => {
                    let v_ = (*v).clone();
                    Ok(Hostname::Host(Some(v_)))
                }
            };
        }

        return Err("Unable to retrieve hostname");
    }
}

fn main() {

    let h0 = HashMap::from(
        [
            ("interface0", Hostname::IpAddr(Some("127.0.0.1".parse().unwrap()))),
            ("interface1", Hostname::IpAddr(Some("::1".parse().unwrap()))),
            ("interface2", Hostname::Host(Some(String::from("example.com")))),
        ]
    );

    println!("h0: {:?}", h0);
    let s: String = serde_yaml::to_string(&h0).unwrap();
    println!("s: {}", s);

    // 1 Hostname
    let s_yml0 = "hostname: 127.0.0.1";
    let s1: Hostname = serde_yaml::from_str(&s_yml0).unwrap();
    println!("s1: {:?}", s1);

    // A dict[key, Hostname]
    let s_yml1 = "---\ninterface0:\n  hostname: 127.0.0.1\ninterface1:\n  hostname: foo.com\ninterface2:\n  hostname: ::1";
    let s2: HashMap<String, Hostname> = serde_yaml::from_str(&s_yml1).unwrap();
    println!("s2: {:?}", s2);

    // An invalid "Hostname" in yaml
    let s_yml2 = "---\ninterface0:\n  hostnam: 127.0.0.1\ninterface1:\n  hostname: foo.com";
    let s3: Result<HashMap<String, Hostname>, serde_yaml::Error> = serde_yaml::from_str(&s_yml2);
    println!("s3: {:?}", s3);

}
