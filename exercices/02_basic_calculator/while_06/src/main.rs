fn main(){
  
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

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}