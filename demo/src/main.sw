contract;

abi MyContract {
    fn test_function() -> bool;
}

impl MyContract for Contract {
    fn test_function() -> bool {
        true
    }
}

use std::bytes::Bytes;
use std::primitive_conversions::u64::*;

/// Returns the digits of a number in reverse order.
/// Eg. 123 -> [3, 2, 1]
/// Handles 0 aswell, 0 -> [0]
fn digits(num: u64) -> Bytes {
    let mut num = num;
    let mut digits = Bytes::new();
    while num > 0 {
        digits.push((num % 10).try_as_u8().unwrap());
        num /= 10;
    }
    digits
}

#[test]
fn test_digits() {
    let mut bytes = Bytes::new();
    bytes.push(3);
    bytes.push(2);
    bytes.push(1);

    assert(digits(123) == bytes);

    let mut bytes = Bytes::new();
    bytes.push(0);
    assert(digits(0) == bytes);


}

