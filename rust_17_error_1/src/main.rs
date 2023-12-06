use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file_0() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // using ? to propagate error to the caller
    // see read_username_from_file_0 for an implementation using: match
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let _ = match read_username_from_file_0() {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let _ = match read_username_from_file() {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("username: {}", read_username_from_file().unwrap());
}
