fn main() {
    factorial(20);
}

pub fn factorial(n: u32) -> u32{
  let mut result: u32 = 1;
  for i in 1..=n{
    result*=i;
  }
  println!("{}", result);
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