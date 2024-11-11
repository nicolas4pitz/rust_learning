use core::num;

fn main() {
    assert_eq!(factorial(4), 125);
}

fn factorial(number: u32) -> u32 {

    if number == 0 {
        1
    } else{
        number * factorial(number - 1)
    }
}