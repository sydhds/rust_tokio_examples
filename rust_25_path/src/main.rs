use std::fmt::Debug;
use std::path::{Path, PathBuf};

fn generic_f1<T>(a1: T)
where
    T: AsRef<Path> + Debug,
{
    let p0: &Path = a1.as_ref(); // type annotation is not mandatory
    println!("parent: {:?}", p0.parent().unwrap());
}

fn generic_f2<T>(a1: T)
where
    T: Into<PathBuf> + Debug,
{
    let p0: PathBuf = a1.into(); // type annotation is not mandatory
    println!("generic join: {:?}", p0.join("bazzzz"));
}

fn main() {
    let p1: &Path = Path::new("/tmp/foo");
    let mut p2: PathBuf = PathBuf::from("/tmp/bar");

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    let p3: &Path = &p2;
    println!("p3: {:?}, p2: {:?}", p3, p2);

    p2.push("baz");
    println!("p2: {:?}", p2); // push modifies the object
    println!("p2 mod: {:?}", p2.join("baz2")); // join return a new PathBuf

    // Can pass types: &Path, PathBuf, &str, String,
    generic_f1(p1);
    generic_f1("/tmp2/foo");
    generic_f1(String::from("/tmp3/foo"));
    generic_f1(PathBuf::from("/tmp4/foo"));

    // Can pass types: &Path, PathBuf, &str, String,
    generic_f2(PathBuf::from("/tmp/foo1"));
    generic_f2(Path::new("/tmp/foo2"));
    generic_f2(String::from("/tmp/foo3"));
    generic_f2("/tmp/foo4");
}
