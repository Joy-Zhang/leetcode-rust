pub fn generate_the_string(n: i32) -> String {
    let m = n as usize;
    if m % 2 == 0 {
        return format!("{}{}", "a".repeat(m - 1), "b");
    } else {
        return "a".repeat(m);
    }
}