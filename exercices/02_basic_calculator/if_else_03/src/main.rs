fn main() {
    assert!(!is_even(1));
    assert!(is_even(2));
    assert!(!is_even(231));
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}