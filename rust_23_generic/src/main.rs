#![allow(dead_code)]
use std::fmt::Display;

#[derive(Debug)]
struct Foo {
    bar: bool,
    baz: String,
    bazz: u32,
}

impl Foo {
    fn display_baz(&self) {
        println!("Foo baz: {}", self.baz);
    }

    // a generic method for our struct
    // require trait Display in order to use "{}" in println!
    // require trait Debug in order to use "{:?}" in println!
    fn display_generic<T>(var: T)
    where
        T: Display,
    {
        println!("Got var: {}", var);
    }
}

// generic function
fn generic<T>(var: T)
where
    T: Display,
{
    println!("Got var: {}", var);
}

// generic struct
#[derive(Debug)]
struct MyStruct<T> {
    test_field: Option<T>,
    name: String,
    age: i32,
}

impl<T> MyStruct<T> {
    fn new(new_age: i32, new_name: String) -> Self {
        MyStruct {
            test_field: None,
            age: new_age,
            name: new_name,
        }
    }
}

// generic struct 2

struct NonDebug {}

#[derive(Debug)]
struct MyStruct2<T> {
    test_field: Option<T>,
    name: String,
    age: i32,
}

impl<T> MyStruct2<T>
where
    T: std::fmt::Debug,
{
    fn new(new_age: i32, new_name: String) -> Self {
        MyStruct2 {
            test_field: None,
            age: new_age,
            name: new_name,
        }
    }
}

fn main() {
    println!("Generic!");

    let a1: u8 = 255;
    let a2: u16 = 16300;

    generic(a1);
    generic(a2);

    let f1 = Foo {
        bar: true,
        baz: "baz: Foo".to_owned(),
        bazz: 645648,
    };

    f1.display_baz();
    Foo::display_generic(a1);

    let s1: MyStruct<u32> = MyStruct {
        test_field: Some(42),
        age: 33,
        name: String::from("22"),
    };
    println!("s1: {:?}", s1);
    let s2: MyStruct<u64> = MyStruct::new(806, String::from("33"));
    println!("s2: {:?}", s2);

    // Does not compile: NonDebug struct does not implement the Debug trait
    //let s3: MyStruct2<NonDebug> = MyStruct2 { test_field: Some(NonDebug {}), age: 33, name: String::from("22") };
    //println!("s3: {:?}", s3);
}
