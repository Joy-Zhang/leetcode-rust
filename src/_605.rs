pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {

    let mut left = n;

    for i in 0..flowerbed.len() {
        if flowerbed[i] == 1 {
            continue;
        }

        if i > 0 && flowerbed[i - 1] == 1 {
            continue;
        }

        if i + 1 < flowerbed.len() && flowerbed[i + 1] == 1 {
            continue;
        }

        flowerbed[i] = 1;
        left -= 1;
        if left <= 0 {
            return true;
        }
    }


    return left <= 0;
    
}