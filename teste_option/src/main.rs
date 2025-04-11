use std::ops::Div;

#[derive(PartialEq, Debug)]
enum DivResult{
    Success(i32),
    DivisionByZero,
    NotDivisible,
}


fn test_divisor(number: i32, divisor: i32) -> DivResult{
    if divisor == 0 {
        DivResult::DivisionByZero
    } else if number % divisor == 0 {
        DivResult::Success(number / divisor)
    } else {
        DivResult::NotDivisible
    }
}

fn main() {
    let number = 10;
    let divisor = 2;

    let result = test_divisor(number, divisor);

    match  result{
       DivResult::DivisionByZero => println!("Division by zero"), 
       DivResult::Success(r) => println!("Result: {}", r),
       DivResult::NotDivisible => println!("Not divisible"),
    }    

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor_zero() {
        assert_eq!(test_divisor(10, 0), DivResult::DivisionByZero);  // Test division by zero
    }

    #[test]
    fn test_valid_division() {
        assert_eq!(test_divisor(10, 2), DivResult::Success((5)));   // Test valid division
    }

    #[test]
    fn test_invalid_division() {
        assert_eq!(test_divisor(10, 3), DivResult::NotDivisible);  // Test invalid divisor
    }

    #[test]
    fn test_negative_division() {
        assert_eq!(test_divisor(10, -2), DivResult::Success(-5));  // Test valid division with a negative divisor
    }
}
