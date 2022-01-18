pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '{' {
            stack.push(c);
        } else if c == '}' {
            if stack.pop() != Some('{') {
                return false;
            }
        } else if c == '[' {
            stack.push(c);
        } else if c == ']' {
            if stack.pop() != Some('[') {
                return false;
            }
        } else if c == '(' {
            stack.push(c);
        } else if c == ')' {
            if stack.pop() != Some('(') {
                return false;
            }
        }
    }
    return stack.is_empty();
}
