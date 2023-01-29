pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums.len() / 2 {
        let freq = nums[i * 2];
        let val = nums[i * 2 + 1];
        for _j in 0..freq {
            result.push(val);
        }
    }

    return result
}