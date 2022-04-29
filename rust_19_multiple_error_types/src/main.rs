use std::num::ParseIntError;
use std::fmt;

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1: Option
    2 * first.parse::<i32>().unwrap() // Generate error 2: Result<i32, ParseIntError>
}

// attempt 1
fn double_first_2(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {

    // => embed Result<...> in Option
    vec
        .first()
        .map(|first| {
            first
                .parse::<i32>()
                .map(|n| 2 * n)
        })
}

// attempt 2
fn double_first_3(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {

    // => invert result type: Option<Result<>> => Result<Option<i32>, ...>
    // if you want to use: ?
    let opt = vec
        .first()
        .map(|first| {
            first
                .parse::<i32>()
                .map(|n| 2 * n)
        });

    // => Return Ok(None) is Option is None else apply a Some??
    opt
        .map_or(Ok(None), |r| r.map(Some))
}

// attempt 3: create our own error
type Result_a3<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DoubleError] invalid first item to double")
    }
}

fn double_first_4(vec: Vec<&str>) -> Result_a3<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result_a3<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// attempt 4: Boxing error
use std::error;

type Result_a4<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[EmptyVec] invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first_5(vec: Vec<&str>) -> Result_a4<i32> {
    vec.first()
        .ok_or(EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| EmptyVec.into())
                .map(|i| 2 * i)
        })
}

fn double_first_5_2(vec: Vec<&str>) -> Result_a4<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print2(result: Result_a4<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // println!("Hello, world!");

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    /*
    println!("The first doubled is {}", double_first(numbers));
    // error 1: input vector is empty
    println!("The first doubled is {}", double_first(empty));
    // error 2: the element does not parse to a number
    println!("The first doubled is {}", double_first(strings));
    */

    /*
    println!("The first doubled is {:?}", double_first_2(numbers));
    // error 1: input vector is empty
    println!("The first doubled is {:?}", double_first_2(empty));
    // error 2: the element does not parse to a number
    println!("The first doubled is {:?}", double_first_2(strings));
    */

    /*
    println!("The first doubled is {:?}", double_first_3(numbers));
    // error 1: input vector is empty
    println!("The first doubled is {:?}", double_first_3(empty));
    // error 2: the element does not parse to a number
    println!("The first doubled is {:?}", double_first_3(strings));
    */

    /*
    print(double_first_4(numbers));
    // error 1: input vector is empty
    print(double_first_4(empty));
    // error 2: the element does not parse to a number
    print(double_first_4(strings));
    */

    print2(double_first_5_2(numbers));
    // error 1: input vector is empty
    print2(double_first_5_2(empty));
    // error 2: the element does not parse to a number
    print2(double_first_5_2(strings));

}

