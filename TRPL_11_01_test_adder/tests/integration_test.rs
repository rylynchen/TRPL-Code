use TRPL_11_01_test_adder::{add_two, add_three};

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}

#[test]
fn it_adds_three() {
    assert_eq!(5, add_three(2));
}