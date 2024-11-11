fn main() {
    //#[test]
    assert_eq!(speed(0, 10, 10), 1);

    assert_eq!(speed(0, 10, 0), 0);
}

fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    if time_elapsed == 0 {
        panic!("Time elapsed não pode ser zero");
    }

    (end - start) / time_elapsed
}