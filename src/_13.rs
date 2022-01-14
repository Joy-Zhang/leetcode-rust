use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {

    let dict: HashMap<&str, i32> = [
        ("M", 1000), 
        ("D", 500),
        ("C", 100),
        ("L", 50),
        ("X", 10),
        ("V", 5),
        ("I", 1)
    ].iter().cloned().collect();

    let mut sum = 0;
    let len = s.len();
    for i in 0..len {
        if i == len - 1 {
            sum += dict[&s[i..i]];
        }
    }

    return sum;
}