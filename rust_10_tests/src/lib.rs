pub fn add(x: &i32, y: &i32) -> i32 {
    // x + y - 1
    adder(x, y)
}

fn adder(x: &i32, y: &i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(adder(&2, &2), 4);
    }
}
