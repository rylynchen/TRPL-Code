fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let a = 5;
    let b = Box::new(5);
    assert_eq!(5, a);
    assert_eq!(5, *b);
}
