pub fn my_sqrt(x: i32) -> i32 {
    fn next(s: f32, i: f32) -> f32 {
        return 0.5 * (i + s / i);
    }
    let mut last = -1;

    let s = x as f32;
    let mut it = s;
    loop {
        it = next(s, it);
        if last == it as i32 {
            break;
        }
        last = it as i32;
    }
    return last;
}