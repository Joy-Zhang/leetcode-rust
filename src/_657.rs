pub fn judge_circle(moves: String) -> bool {
    
    let mut y_stack: Vec<char> = Vec::new();
    let mut x_stack: Vec<char> = Vec::new();
    
    for c in moves.chars() {
        
        match c {
            'U' | 'D' => {
                let last = y_stack.last();
                if last.is_none() {
                    y_stack.push(c);
                } else {
                    let last_val = last.unwrap();
                    if c == *last_val {
                        y_stack.push(c);
                    } else {
                        y_stack.pop();
                    }

                }
            }
            'L' | 'R' => {
                let last = x_stack.last();
                if last.is_none() {
                    x_stack.push(c);
                } else {
                    let last_val = last.unwrap();
                    if c == *last_val {
                        x_stack.push(c);
                    } else {
                        x_stack.pop();
                    }

                }
            }
            _ => {}
        }
        
    }

    return y_stack.is_empty() && x_stack.is_empty();


        
}