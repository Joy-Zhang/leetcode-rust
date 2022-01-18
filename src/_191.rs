pub fn hammingWeight (n: u32) -> i32 {
    let mut count = 0;
    let mut num = n;
    while num != 0 {
        count += (num & 0x01) as i32;
        num >>= 1;
    }
    return count;
}