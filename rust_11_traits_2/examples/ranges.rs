use std::fmt::{Debug, Display};
use std::ops::{Range, RangeBounds, RangeFrom};

fn main() {
    let r1: Range<u32> = 0..2;
    let r2 = 0u32..;
    let r3 = ..5u32;

    println!("Calling take_range_1 on r1");
    // Pass a standard range (https://doc.rust-lang.org/std/ops/struct.Range.html)
    take_range_1(r1.clone());
    // This will not compile as r2 is RangeFrom
    // https://doc.rust-lang.org/std/ops/struct.RangeFrom.html
    // take_range_1(r2);

    println!("Calling take_range_2 on r2");
    take_range_2(r2.clone());

    // Can use take_range_3 as a generic function
    println!("Calling take_range_3 on r1");
    take_range_3(r1.clone());
    println!("Calling take_range_3 on r2");
    take_range_3(r2.clone());

    // Will not compile as r3 is RangeTo
    // https://doc.rust-lang.org/std/ops/struct.RangeTo.html
    // and RangeTo does not implement Iterator trait
    // println!("Calling take_range_3 on r3");
    // take_range_3(r3.clone());

    println!(
        "Calling take_range_4 on r1: {}",
        take_range_4(r1.clone(), 4)
    );
    println!(
        "Calling take_range_4 on r2: {}",
        take_range_4(r2.clone(), 4)
    );
    println!(
        "Calling take_range_4 on r3: {}",
        take_range_4(r3.clone(), 4)
    );
    println!(
        "Calling take_range_4 on another range: {}",
        take_range_4(0..7u64, 4)
    );
    // Will not compile as the range is RangeBounds<u64> while index is u32
    // println!(
    //     "Calling take_range_4 on another range: {}",
    //     take_range_4(0..7u64, 4u32)
    // );

    // With take_range_5 and because u32 can be converted to u64 it works :)
    println!(
        "Calling take_range_5 on another range: {}",
        take_range_5(0..7u64, 4u32)
    );
    println!(
        "Calling take_range_6 on another range: {}",
        take_range_6(0..7u64, 4u32)
    );
}

fn take_range_1(r: Range<u32>) {
    for i in r.take(10) {
        println!("i: {}", i);
    }
}

fn take_range_2(r: RangeFrom<u32>) {
    for i in r.take(10) {
        println!("i: {}", i);
    }
}

// A generic function over range with trait RangeBounds + Iterator
fn take_range_3<R, I>(r: R)
where
    R: RangeBounds<I> + Iterator,
    <R as Iterator>::Item: Debug + Display,
    I: Debug + Display, // This line is not mandatory but more explicit :)
{
    for i in r.take(10) {
        println!("i: {}", i);
    }
}

// A generic function over range with only trait RangeBounds
fn take_range_4<R, I>(r: R, i: I) -> bool
where
    R: RangeBounds<I>,
    I: num::Integer,
{
    r.contains(&i)
}

// Another version of take_range_4
fn take_range_5<R, I, IT>(r: R, i: IT) -> bool
where
    R: RangeBounds<I>,
    I: num::Integer,
    IT: Into<I>,
{
    r.contains(&i.into())
}

// Another version of take_range_5
fn take_range_6<R, I, IT>(r: R, i: IT) -> bool
where
    R: RangeBounds<I>,
    I: num::Integer + From<IT>,
{
    r.contains(&I::from(i))
}
