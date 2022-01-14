pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last = nums.len() - 1;

    let mut i = 0;

    while i < last {

        if nums[i] == 0 {
            for _j in i..last {
                nums.swap(_j, _j + 1)
            }
            last -= 1
        } else {
            i += 1;
        }
    }
}
