pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    let mut table = start ^ goal;
    let mut count = 0;
    while table > 0 {
        if table & 1 == 1 {
            count += 1;
        }
        table >>= 1;
    }
    return count;
}