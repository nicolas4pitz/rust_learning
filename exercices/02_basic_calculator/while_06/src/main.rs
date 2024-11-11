fn main() {
    assert_eq!(factorial(5), 120);
}

fn factorial (number: u32) -> u32 {
    let mut contador = 1;
    let mut result: u32 = 1;
    while contador <= number {
        result *= contador;
        //println!("result: {}", result);
        contador+=1;
    }
    result
}