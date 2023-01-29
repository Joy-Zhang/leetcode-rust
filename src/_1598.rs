pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut depth = 0;
    for i in 0..logs.len() {
        let log = &logs[i];
        if log == "../" {
            if depth > 0 {
                depth -= 1;
            }
        } else if log == "./" {
            continue;
        } else {
            depth += 1;
        }
    }
    return depth;
}