
// From
// possiblerust.com/guide/what-can-coerce-and-where-in-rust#code-1

fn ref_downgrade_coercions() {

    struct RefHolder<'a> {
        x: &'a i64,
    }

    impl<'a> RefHolder<'a> {
        fn new(x: &'a i64) -> RefHolder<'a> {
            RefHolder { x }
        }
    }

    fn print_num(y: &i64) {
        println!("y: {}", y);
    }

    let mut x = 10; // x type: i64
    let y = &mut x; // y type: &mut i64
    let z = RefHolder::new(y); // ref downgrade from &mut i64 -> &i64
    print_num(y); // same here
    println!("z.x: {}", z.x); // using ref
}

fn deref_coercion() {

    use std::ops::Deref;
    use std::ops::DerefMut;

    #[derive(Debug)]
    struct DummyWrapper<T> {
        t: T,
    }

    impl<T> Deref for DummyWrapper<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            return &self.t;
        }
    }

    // DerefMut requires Deref as a supertrait
    impl<T> DerefMut for DummyWrapper<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            return &mut self.t;
        }
    }

    let s = String::from("foo");
    let bs = Box::new(s);
    // deref coercion: calling is_empty on Box, that deref to &String
    println!("is String in dw empty: {}, content: {:?}", bs.is_empty(), bs);

    let s2 = String::from("");
    let mut dw = DummyWrapper { t: s2 };
    // deref coercion: calling is_empty on DummyWrapper, that deref to &String
    println!("is String in dw empty: {} - content: {:?}", dw.is_empty(), dw);
    // deref mut coercion
    dw.push('a');
    println!("is String in dw empty: {} - content: {:?}", dw.is_empty(), dw);

    let bdw = Box::new(DummyWrapper { t: String::from("foo bar baz") });
    // deref coercion works even in chain: first deref for Box, then deref for DummyWrapper
    println!("is String in bdw empty: {} - content: {:?}", bdw.is_empty(), bdw);
}

fn raw_pointers_coercion() {

    #[derive(Debug)]
    struct PtrHandle {
        ptr: *const i32,
    }

    let mut x = 5;
    let ptr = &mut x as *mut i32; // raw pointer to i32
    let handle = PtrHandle { ptr }; // ptr coercion from *mut i32 -> *const i32
    println!("handle: {:?}", handle);
}

fn ref_raw_pointers_coercion() {

    #[derive(Debug)]
    struct ConstHandle<T> {
        ptr: *const T,
    }

    #[derive(Debug)]
    struct MutHandle<T> {
        ptr: *mut T,
    }

    let mut x = 5;
    let c_handle = ConstHandle { ptr: &x }; // &i32 -> *const i32
    let m_handle = MutHandle { ptr: &mut x }; // &mut i32 -> *mut i32

    println!("c_handle: {:?}", c_handle);
    println!("m_handle: {:?}", m_handle);
}

fn func_coercion() {

    fn takes_func_ptr(f: fn(i32) -> i32, i: i32) -> i32 {
        f(i)
    }

    // Another way to write takes_func_ptr using trait
    // Note here that there is no coercion
    // Closures implement at least FnOnce trait (FnMut if no move in captured env, Fn if no mut / move in captured env)
    fn takes_func_ptr2<F>(f: F, i: i32) -> i32 where F: FnOnce(i32) -> i32 {
        f(i)
    }
    // As described below, our closure does not mutate / move anything so impl trait: Fn
    fn takes_func_ptr3<F>(f: F, i: i32) -> i32 where F: Fn(i32) -> i32 {
        f(i)
    }

    let my_func = |n| { n+2 };

    println!("takes_func_ptr result (i=2): {}", takes_func_ptr(my_func, 0));
    println!("takes_func_ptr result (i=17): {}", takes_func_ptr2(my_func, 17));
    println!("takes_func_ptr result (i=17): {}", takes_func_ptr3(my_func, 21));
}

const S0: &str = "barbaz";

fn subtype_coercion<'a>() {

    struct FnHolder {
        f: fn(&'static str) -> i32,
    }

    fn number_for_name<'a>(name: &'a str) -> i32 {
        match name {
            "Jim" => 42,
            _ => 5,
        }
    }

    let holder = FnHolder { f: number_for_name };

    // subtype coercion from lifetime 'a to 'static
    // this is fine as: 'static lifetime lasts longer than 'a lifetime
    println!("FnHolder (arg: 'Jim'): {}", (holder.f)("Jim"));
    println!("FnHolder (arg: 'Bob'): {}", (holder.f)("Bob"));

    // Does not compile as
    let _s: &'a str = "foo";
    // println!("FnHolder (arg: variable s): {}", (holder.f)(s));
    // This is fine
    println!("FnHolder (arg: variable S0: {}): {}", S0, (holder.f)(S0));
}

fn never_coercion() {

    struct Value {
        x: bool,
        y: String,
    }

    fn never() -> ! {
        loop{
        }
    }

    // let x = never(); // uncomment to enable it - will require Ctrl-C to exit the program
    // uncomment to see that it compiles
    /*
    let _v = Value {
        x: todo!("Not yet!"),
        y: unimplemented!("Uh no!!"),
    };
    */
}

fn slice_coercion() {

    #[derive(Debug)]
    struct SliceHolder<'a> {
        slice: &'a [i32],
    }

    // Not valid, compiler will ask to add a lifetime (see SliceHolder<'a>)
    /*
    #[derive(Debug)]
    struct SliceHolder2 {
        slice: &[i32],
    }
    */

    let nums = [1, 2, 3, 4, 5];
    let holder = SliceHolder { slice: &nums };
    println!("holder: {:?}", holder);
}

fn trait_obj_coercion() {

    trait HasInt {
        fn get(&self) -> i32;
    }

    struct IntHolder {
        x: i32,
    }

    struct Int8Holder {
        x: i8,
    }

    impl HasInt for IntHolder {
        fn get(&self) -> i32 {
            return self.x;
        }
    }

    impl HasInt for Int8Holder {
        fn get(&self) -> i32 {
            return self.x as i32;
        }
    }

    fn print_int(x: &dyn HasInt) {
        println!("int value: {}", x.get());
    }

    let i1 = IntHolder { x: 33 };
    let i2 = Int8Holder { x: 127 };
    // coercion from &IntHolder -> &dyn HastInt
    print_int(&i1);
    // coercion from &Int8Holder -> &dyn HastInt
    print_int(&i2);

}

fn least_upper_bound_coercion() {

    let i = 500;

    let x: i8 = 10;
    let mut y: i8 = 20;

    let mut z_ = 25;
    let mut _z: Box<& mut i8> = Box::new(&mut z_);


    let v = match i {
        i if i < 5 => { &x }, // return here is: & i8
        i if i >= 5 && i < 500 => { &mut y }, // return here is &mut i8 -> & i8
        i if i >= 500 => { *_z },  // &mut i8 -> & i8
        _ => {
            panic!("??");
        }
    };

    let v2: &i8 = v;
    println!("v2: {:?}", v2);

}

fn main() {

    /*
     * In Rust, type conversion can be explicit:
     * From/Into traits -> infallible convert
     * TryFrom/TryInto traits -> fallible convert
     * AsRef trait -> ??
     * AsMut trait -> ??
     * Borrow -> ??
     * ToOwned -> ??
     *
     * Coercions are implicit (note: casts done with the as keyword are explicit)
     * There are multiple type of coercions
     */

    // 1- Reference downgrade coercions: &mut T -> &T
    ref_downgrade_coercions();

    // 2- Deref coercion: most of the time for smart pointers
    //                 it comes from traits: Deref / DerefMut
    // Example: Box<T> implements Deref<Target: T> so we can call method of T on a Box<T>
    deref_coercion();

    // 3- Raw pointers coercion: rust raw pointers coercion from *mut T -> *const T
    raw_pointers_coercion();

    // 4- Reference and raw pointers coercions
    // &T -> *const T
    // &mut T -> *mut T
    ref_raw_pointers_coercion();

    // 5- Function pointers coercion
    // closure -> fn
    // only if closure does not capture env
    func_coercion();

    // subtype coercion (e.g. for lifetimes)
    // 'a -> 'static (see comments in function)
    subtype_coercion();

    // never coercion
    // ! (never type) -> Anything
    never_coercion();

    // slice coercion (part of a set of unsized coercion)
    // Array -> slice
    // [T; n] -> [T]
    // Vec<T> -> [T] is Deref coercion, Array do not implement Deref thus having this slice coercion
    slice_coercion();

    // trait object coercion
    // trait A -> &dyn A
    trait_obj_coercion();

    // trailing unsized coercion
    // ??

    // least upper bound coercion
    // coercion when multiple branch occurs
    least_upper_bound_coercion();

    // transitive coercion
    // if A -> B and B -> C then A -> C
    // See deref coercion when we use Box<DummyWrapper<String>>



}