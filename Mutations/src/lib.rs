#![allow(non_snake_case)]

#[no_mangle]
pub extern "C" fn mult_numbers(left: u64, right: u64) -> u64{
    left * right
}

#[no_mangle]
pub extern "C"  fn add_numbers(left: u64, right: u64) -> u64 {
    left + right
}

// Unit test support via "Test Rust" configuration
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_numbers_works() {
        let result = add_numbers(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn mult_numbers_works() {
        let result = mult_numbers(3, 3);
        assert_eq!(result, 9);
    }

}
