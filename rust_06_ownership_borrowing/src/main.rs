fn str_owner(s: String) {
    println!("s: {}", s); // Warning: take ownership of s
}

fn str_owner_ref(s: &String) {
    println!("s: {}", s);
}

fn str_owner_ref_mut(s: &mut String) {
    s.push_str(" ++mut");
    println!("s: {}", s);
}

fn main() {
    // println!("Hello, world!");

    let x = 5;
    let y = x;
    println!("x: {}", x); // Ok to do this, value has been copied
    println!("y: {}", y);

    let s1 = String::from("Ownership 1");
    let s2 = s1;
    // Cannot do this, s2 owns the data
    //println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s3 = String::from("Hello world!");
    str_owner(s3);
    // println!("s3: {}", s3); // cannot use s3 here, see str_owner comment

    let s4 = String::from("Hello world 4!");
    str_owner_ref(&s4);
    println!("s4: {}", s4);

    let mut s5 = String::from("Hello world 5!");
    str_owner_ref_mut(&mut s5);
    println!("s5: {}", s5);

    // clone (copy) instead of move (note: this make a copy of data)
    let s6 = s2.clone();
    println!("s2: {}", s2);
    println!("s6 (s2 clone): {}", s6);
}
