pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut t: Option<i32> = None;
    let len = nums.len();
    let mut op = 0;
    let mut move_left = 0;
    for _ in 0..len {
        if Some(nums[op]) == t {
            move_left += 1
        } else {
            // 向左复制
            for j in op..len {
                nums[j - move_left] = nums[j];
            }

            op -= move_left;

            t = Some(nums[op]);

            move_left = 0;
        }

        op += 1

    }
    op -= move_left;
    return op as i32;
} 