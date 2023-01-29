pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut sorted = nums.to_vec();
    sorted.sort();
    let mut target = -40000;
    let mut found = 0;
    let mut i = 0;

    while i < sorted.len() {

        if sorted[i] != target {
            if found == 1 {
                break;
            }
            target = sorted[i];
            found = 1;
        } else {
            found += 1;
        }


        i += 1;
    }

    return target;
}