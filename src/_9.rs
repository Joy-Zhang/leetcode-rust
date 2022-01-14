

pub fn is_palindrome(x: i32) -> bool {

    if x < 0 {
        return false;
    }

    fn is_palindrome_r(digits: &[i32]) -> bool {
        let len = digits.len();
        if len <= 1 {
            return true;
        }
        if digits.first() == digits.last() {
            let last = len - 1;
            return is_palindrome_r(&digits[1..last]);
        } else {
            return false;
        }
    }

    let mut digits = Vec::new();
    let mut current = x;
    while current > 0 {
        let _digit = current % 10;
        digits.push(_digit);
        current = current / 10;
    }
    return is_palindrome_r(&digits[..]);
}