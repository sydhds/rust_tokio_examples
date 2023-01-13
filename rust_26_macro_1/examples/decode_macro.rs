use base64::decode;

macro_rules! decode {
    ($a: expr) => {
        {
            std::str::from_utf8(&decode($a).unwrap()).unwrap()
        }
    }
}

macro_rules! decode2 {
    ($a: expr) => {
        {
            unsafe { std::str::from_utf8_unchecked(&decode($a).unwrap()) }
        }
    }
}

fn main() {
    let name = "Zm9vYmFyYmF6";
    println!("name raw: {}", name);
    println!("name decoded: {}", decode!(name));
    println!("name decoded: {}", decode2!(name));
}