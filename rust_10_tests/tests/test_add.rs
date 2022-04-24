use rust_10_tests::add;

#[test]
fn it_works_ex() {
    assert_eq!(add(&2, &2), 4);
}

#[test]
fn it_works_ex_0() {
    assert_eq!(add(&0, &0), 0);
}
