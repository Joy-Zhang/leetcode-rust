/*
 0,1,2,3,4,5,6, 7
[0,1,3,5,6,8,12,17]

17 17 16 14 12 11 9 5

j(?, 7|17, )
j(6|12, 7|17, 5)? -> j([0,1,2,3,4,5], 6|12, [4,5,6])? -> j(5|8, 6|12, 4)?



[0,1]

j(0|0,1)?

*/
pub fn can_cross(stones: Vec<i32>) -> bool {

    fn can_cross_r(stones: &Vec<i32>, i_from: usize, i_to: usize, distance: i32, r: usize) -> bool {


        println!("{}can_cross_r({}|{},{}|{},{})", " ".repeat(r), i_from, stones[i_from], i_to, stones[i_to], distance);


        if i_from == 0 {
            return distance == 1
        }

        if stones[i_to] - stones[i_from] != distance {
            return false
        }
        


        for d in [distance - 1, distance, distance + 1] {
            for i in (0..i_from).rev() {
                if can_cross_r(stones, i, i_from, d, r + 1) {
                    return true;
                }
            }
        }

        return false;

    }

    let i_last = stones.len() - 1;

    for i in (0..i_last).rev() {
        if can_cross_r(&stones, i, i_last, stones[i_last] - stones[i], 1) {
            return true;
        }
    }



    return false;



    
    
}