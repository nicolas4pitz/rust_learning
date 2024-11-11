fn main() {
    assert_eq!(speed(0, 10, 10), 1);
    assert_eq!(speed(40, 80, 20), 2);
    assert_eq!(speed(15, 20, 1), 1);
}

fn speed (start: u32, end: u32, time_elapsed: u32) -> u32{
    let distance = end - start;
    distance / time_elapsed
}