use std::fmt;
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1: Option
    2 * first.parse::<i32>().unwrap() // Generate error 2: Result<i32, ParseIntError>
}

// attempt 1
fn double_first_2(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    // => embed Result<...> in Option
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

// attempt 2
fn double_first_3(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    // => invert result type: Option<Result<>> => Result<Option<i32>, ...>
    // if you want to use: ?
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    // => Return Ok(None) is Option is None else apply a Some??
    opt.map_or(Ok(None), |r| r.map(Some))
}

// attempt 3: create our own error
type ResultA3<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DoubleError] invalid first item to double")
    }
}

fn double_first_4(vec: Vec<&str>) -> ResultA3<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}

// attempt 4: Boxing error
use std::error;

type ResultA4<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[EmptyVec] invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first_5(vec: Vec<&str>) -> ResultA4<i32> {
    vec.first().ok_or(EmptyVec.into()).and_then(|s| {
        s.parse::<i32>() // Result<i32, ParseIntError>
            .map_err(|e| e.into()) // Result<i32, Box<dyn Error>>
            .map(|i| 2 * i)
    })
}

fn double_first_5_2(vec: Vec<&str>) -> ResultA4<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn main() {
    // println!("Hello, world!");

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    // Step 1: Unwrap
    println!("[1] The first doubled is {}", double_first(numbers.clone()));
    // error 1: input vector is empty - will panic
    // println!("The first doubled is {}", double_first(empty));
    // error 2: the element does not parse to a number - will panic
    // println!("The first doubled is {}", double_first(strings));

    // Step 2: Return Option<Result<..>>
    println!(
        "[2] The first doubled is {:?}",
        double_first_2(numbers.clone())
    );
    println!(
        "[2b] The first doubled is {:?}",
        double_first_2(empty.clone())
    );
    println!(
        "[2c] The first doubled is {:?}",
        double_first_2(strings.clone())
    );

    // Step 3: Return Result<Option<..>>
    println!(
        "[3] The first doubled is {:?}",
        double_first_3(numbers.clone())
    );
    println!(
        "[3b] The first doubled is {:?}",
        double_first_3(empty.clone())
    );
    println!(
        "[3c] The first doubled is {:?}",
        double_first_3(strings.clone())
    );

    // Step 4: Return custom error (struct DoubleError)
    // Note: no distinction between ParseIntError & "empty vec error"
    println!(
        "[4] The first doubled is {:?}",
        double_first_4(numbers.clone())
    );
    println!(
        "[4b] The first doubled is {:?}",
        double_first_4(empty.clone())
    );
    println!(
        "[4c] The first doubled is {:?}",
        double_first_4(strings.clone())
    );

    // Step 5: Return Box<dyn Error>
    println!(
        "[5] The first doubled is {:?}",
        double_first_5(numbers.clone())
    );
    println!(
        "[5b] The first doubled is {:?}",
        double_first_5(empty.clone())
    );
    println!(
        "[5c] The first doubled is {:?}",
        double_first_5(strings.clone())
    );

    // Step 5_2: Return Box<dyn Error>
    println!(
        "[5_2] The first doubled is {:?}",
        double_first_5_2(numbers.clone())
    );
    println!(
        "[5_2b] The first doubled is {:?}",
        double_first_5_2(empty.clone())
    );
    println!(
        "[5_2c] The first doubled is {:?}",
        double_first_5_2(strings.clone())
    );
}
