// #![feature(step_trait)]

use std::fmt::{Debug, Display};
use std::ops::{Range, RangeBounds, RangeFrom, RangeInclusive};

fn main() {
    let r1: Range<u32> = 0..2;
    let r1_2: RangeInclusive<u32> = 0..=4;
    let r2 = 0u32..; // type: RangeFrom<u32>
    let r3 = ..5u32; // type: RangeTo<u32>

    println!("Calling take_range_1 on r1");
    // Pass a standard range (https://doc.rust-lang.org/std/ops/struct.Range.html)
    take_range_1(r1.clone());
    // This will not compile as r1_2 is RangeInclusive && r2 is RangeFrom
    // https://doc.rust-lang.org/std/ops/struct.RangeFrom.html
    // take_range_1(r1_2.clone());
    // take_range_1(r2);

    println!("Calling take_range_2 on r2");
    take_range_2(r2.clone());

    // Can use take_range_3 as a generic function
    println!("Calling take_range_3 on r1");
    take_range_3(r1.clone());
    println!("Calling take_range_3 on r1_2");
    take_range_3(r1_2.clone());
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
    println!("Calling take_range_4 on r3: {}", take_range_4(r3, 4));
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

    // Function returning ranges
    let rm_1 = make_range_2(5, 7, true);
    let rm_1_2 = make_range_2(5, 7, false);
    println!("Iterating over rm_1 (from make_range_2 inclusive):");
    rm_1.for_each(|i| println!("i: {}", i));
    println!("Iterating over rm_1_2 (from make_range_2 non inclusive):");
    rm_1_2.for_each(|i| println!("i: {}", i));

    // Only on Rust nightly
    /*
    let rm_1_3 = make_range_3(5, 7, false);
    println!("Iterating over rm_1_3 (from make_range_3 non inclusive):");
    rm_1_3.for_each(|i| println!("i: {}", i));
    */

    // Cannot clone Box<Iterator<...>>
    // let rm_1_2_clone = rm_1_2.clone();

    let rm_c_1 = make_range_clone_2(5, 7, true);
    let rm_c_1_2 = make_range_clone_2(5, 7, false);
    println!("Iterating over clone() rm_c_1 (from make_range_clone_2 inclusive):");
    rm_c_1.clone_box().for_each(|i| println!("i: {}", i));
    println!("Iterating over cloned rm_c_1_2 (from make_range_clone_2 non inclusive):");
    rm_c_1_2.clone_box().for_each(|i| println!("i: {}", i));
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

// This will not compile: `RangeBounds` cannot be made into an object
// According to: https://doc.rust-lang.org/reference/items/traits.html#object-safety
// Generics are not compatible with vtables.
// And RangeBounds has method: fn contains<U>(&self, item: &U) -> bool
/*
fn make_range<T>(start: T, end: T, is_inclusive: bool) -> Box<dyn RangeBounds<T>> {
    if is_inclusive {
        Box::new(start..=end)
    } else {
        Box::new(start..end)
    }
}
*/

fn make_range_2(start: i32, end: i32, is_inclusive: bool) -> Box<dyn Iterator<Item = i32>> {
    if is_inclusive {
        Box::new(start..=end)
    } else {
        Box::new(start..end)
    }
}

// Require unstable feature
// https://doc.rust-lang.org/std/iter/trait.Step.html
/*
fn make_range_3<T: num::Integer + std::iter::Step + 'static>(
    start: T,
    end: T,
    is_inclusive: bool,
) -> Box<(dyn Iterator<Item = T> + 'static)> {
    if is_inclusive {
        Box::new(start..=end)
    } else {
        Box::new(start..end)
    }
}
*/

// Note:
// Clone (https://doc.rust-lang.org/std/clone/trait.Clone.html) has supertrait Sized
// but Box<dyn Trait> is not Sized
trait CloneIterator: Iterator {
    fn clone_box(&self) -> Box<dyn CloneIterator<Item = Self::Item>>;
}

// Implement our special trait for all Cloneable Iterators
impl<T> CloneIterator for T
where
    T: 'static + Iterator + Clone,
{
    fn clone_box(&self) -> Box<dyn CloneIterator<Item = Self::Item>> {
        Box::new(self.clone())
    }
}

fn make_range_clone_2(
    start: i32,
    end: i32,
    is_inclusive: bool,
) -> Box<dyn CloneIterator<Item = i32>> {
    if is_inclusive {
        Box::new(start..=end)
    } else {
        Box::new(start..end)
    }
}
