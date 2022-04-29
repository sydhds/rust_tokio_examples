use std::mem;

// from:
// http://doc.rust-lang.org/rust-by-example/trait/dyn.html

struct Sheep {
}
struct Cow {
    has_milk: bool,
}

struct Cat {
    is_garfield: bool,
    like_to_be_pet: bool,
}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "mooooo!".to_string()
    }
}

impl Animal for Cat {
    fn noise(&self) -> String {
        "Meow!".to_string()
    }
}

// Note: Box -> value alloc on the heap (default is the Stack)
//       Box is a smart pointer
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow { has_milk: false })
    }
}


fn main() {
    // println!("Hello, world!");
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("Random animal... noise: {}", animal.noise());

    // Box examples
    let margo = Box::new(Cow { has_milk: true }); // heap allocated
    let margo2: Box<Cow> = Box::new(Cow { has_milk: false }); // same + type annotation
    // let margo2_2: Box<dyn Animal> = Box::new(Cow { has_milk: true }); // type annotation with a Trait
    let margo2_2: Box<dyn Animal> = Box::new(Cat { is_garfield: false, like_to_be_pet: true }); // type annotation with a Trait
    let mut margo3: Cow = Cow { has_milk: true }; // stack allocated
    let mut margo4: Cat = Cat { is_garfield: true, like_to_be_pet: false };

    println!("[margo] Size on stack: {}", mem::size_of_val(&margo)); // pointer size => size == 8
    println!("[margo2] Size on stack: {}", mem::size_of_val(&margo2)); // same
    println!("[margo2_2] Size on stack: {}", mem::size_of_val(&margo2_2)); // pointer + pointer to a vtable of function pointers (== 2 * 8)
    println!("[margo3] Size on stack: {}", mem::size_of_val(&margo3)); // 1 boolean => size == 1

    println!("[margo3] has milk: {}", margo3.has_milk);
    // Copy the data from margo2 to margo3 (so copy from heap to stack)
    margo3 = *margo2;
    println!("[margo3] has milk: {}", margo3.has_milk);

    // margo4 = *margo2_2; // need as_any && downcast_ref (see stackoverflow 33687447)

    // Note: uncomment to get a stack overflow error (size of array is too big on the stack)
    //       stack size is usually 1Mb on Windows, 8Mb on Linux
    /*
    let a: [i32; 10000000] = [0; 10000000];
    println!("a[0]: {}", a[0]);
    println!("a[10 000 000]: {}", a[10000000-1]);
    */
}
