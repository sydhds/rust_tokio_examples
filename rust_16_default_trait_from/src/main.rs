#[derive(Debug)]
pub struct FooArgs {
    a: f64,
    b: i32,
}

impl Default for FooArgs {
    fn default() -> Self {
        FooArgs { a: 1.0, b: 1 }
    }
}

impl From<f64> for FooArgs {
    fn from(a: f64) -> Self {
        // using ..Self::default() means: use the value of Self::default()
        // for remaining fields to init (here it's only for field: b)
        Self {
            a,
            ..Self::default()
        }
    }

    // Another way to use Default trait
    /*
    fn from(a: f64) -> Self {
        Self { a: a, b: Self::default().b }
    }
    */

    // without using Default trait
    /*
    fn from(a: f64) -> Self {
        Self { a, b: 1 }
    }
    */
}

impl From<(f64, i32)> for FooArgs {
    // Convert a Tuple (f64, i32) to a FooArgs struct
    fn from((a, b): (f64, i32)) -> Self {
        Self { a, b }
    }
}

pub fn foo<A>(args: A) -> f64
where
    A: Into<FooArgs>,
{
    let args_ = args.into();
    args_.a * (args_.b as f64)
}

fn main() {
    println!("Hello, world!");
    // require the trait From<(float, integer)> to be implemented
    println!("{}", foo((2.0, 6)));
    // require the trait From<float> to be implemented
    // Note that the From<float> implementation also use the trait: Default implemented for FooArgs
    println!("{}", foo(5.0));
    println!("{:?}", FooArgs::default());
    let fd: FooArgs = Default::default();
    println!("{:?}", fd);
}
