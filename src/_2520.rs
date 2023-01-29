pub fn count_digits(num: i32) -> i32 {
    let mut mut_num = num;
    let mut count = 0;
    while mut_num > 0 {
        let digit = mut_num % 10;
        if num % digit == 0 {
            count = count + 1;
        }
        mut_num = mut_num / 10;
    }

    return count;

}