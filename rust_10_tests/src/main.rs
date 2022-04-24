use rust_10_tests::add;

pub fn sub(x: &i32, y: &i32) -> i32 {
    x - y
}


#[cfg(test)]
mod tests {

    // allow access to add() in lib.rs
    use rust_10_tests::add;
    // allow access to sub() in current file
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(&2, &2), 4);
    }

    #[test]
    fn it_works_0() {
        assert_eq!(add(&0, &0), 0);
    }

    #[test]
    fn it_works_sub() {
        assert_eq!(sub(&2, &2), 0);
    }

    #[test]
    fn dummy() {
        assert_ne!(add(&2, &2), 3);
        let b = true;
        assert!(b);
    }

}

fn main() {
    println!("Hello, world!");
    let a = 2;
    let b = 2;
    println!("Computing: {} + {} = {}", a, b, add(&a, &b));
    println!("Computing: {} - {} = {}", a, b, sub(&a, &b));
}
