use std::borrow::Cow;
use std::path::Path;

// Cow -> Clone On Write

struct MyCow<'a> {
    c: Cow<'a, str>
}

#[derive(Debug)]
struct Token<'a> {
    raw: Cow<'a, str>
}

impl <'a> Token<'a> {
    pub fn new<S>(raw: S) -> Self where S: Into<Cow<'a, str>> {
        Self {
            // This allow:
            // passing &str (&str -> Cow already implemented, will return Cow::Borrowed)
            // passing String (String -> Cow already implemented, will return Cow::Owned)
            raw: raw.into()
        }
    }
}

fn main() {

    // First example of Cow, to_string_lossy returns a Cow<str>
    let path = Path::new("foo.txt");
    let c = path.to_string_lossy();
    match c {
        Cow::Borrowed(_str_ref) => {
            // Path is valid utf-8 -> return a reference to original data
            println!("path was valid UTF-8"); },
        Cow::Owned(_new_string) => {
            // Path is not valid -> clone original data and replace invalid char with '?'
            println!("path was not valid UTF-8"); }
    }

    // 2nd example (from rust doc: Cow on array of i32)
    let array1 = [0, 1, 2];
    let array1_slice = &array1[..];
    let mut c2 = Cow::from(array1_slice);
    abs_all(&mut c2); // c2 is not modified
    if let Cow::Borrowed(_) = c2 { println!("c2 is still borrowed!"); }

    let array1 = [0, 1, -2];
    let array1_slice = &array1[..];
    let mut c2_1 = Cow::from(array1_slice);
    abs_all(&mut c2_1); // here c2_1 is modified (because of value -2)
    if let Cow::Owned(_) = c2_1 { println!("c2 is now owned!"); }

    // Cow in a struct
    let path = "bar.txt";
    let mycow = MyCow { c: replace_txt_ext_to_md_ext(path) };
    println!("mycow: {}", mycow.c);
    if let Cow::Owned(_) = mycow.c { println!("mycow.c is owned!"); }

    let path = "bar.jpg";
    let mycow = MyCow { c: replace_txt_ext_to_md_ext(path) };
    println!("mycow: {}", mycow.c);
    if let Cow::Borrowed(_) = mycow.c { println!("mycow.c is borrowed!"); }

    // Another example with Token struct + threads
    let token1 = Token::new("12345");
    let token2 = Token::new(String::from("9876"));

    let s = String::from("FOOBAR");
    let sr = &s[..];
    let token3 = Token::new(sr);

    std::thread::spawn(move || {
        println!("token2: {:?}", token2);
    }).join().unwrap();

    // Note: this work for token1 as the lifetime is: 'static
    std::thread::spawn(move || {
        println!("token1: {:?}", token1);
    }).join().unwrap();

    // Note: does not work, lifetime is non 'static
    /*
    std::thread::spawn(move || {
        println!("token3: {:?}", token3);
    }).join().unwrap();
    */

    // Another example
    let ic_1 = how_many_items(0);
    if let Cow::Borrowed(_) = ic_1 { println!("ic_1 is borrowed, value: {}", ic_1); }
    let ic_2 = how_many_items(5);
    if let Cow::Owned(_) = ic_2 { println!("ic_2 is owned, value: {}", ic_2); }

}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn replace_txt_ext_to_md_ext(s: &str) -> Cow<str> {
    // bar.txt -> bar.md
    // bar.jpeg -> bar.jpeg

    if s.ends_with(".txt") {
        Cow::Owned(s.replace(".txt", ".md"))
    } else {
        Cow::Borrowed(s)
    }
}

fn how_many_items(count: usize) -> Cow<'static, str> {
    match count {
        0 => "No more items".into(),
        1 => "Last item available".into(),
        _ => format!("{} items remaining", count).into(),
    }
}

