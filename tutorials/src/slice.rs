fn main() {
    let s = [1, 2, 3];
    let slice1 = &s[0..2];
    let slice2 = &[1, 2];
    assert_eq!(slice1, slice2);
}
