use std::collections::HashMap;

pub fn has_groups_size_x(deck: Vec<i32>) -> bool {

    fn _cd(first: usize, second: usize) -> Vec<usize> {
        let mut cd = Vec::new();
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }
    
        let mut i_cd = 2;
        while i_cd <= min {
            if min % i_cd == 0 && max % i_cd == 0 {
                cd.push(i_cd);
            }
            i_cd += 1;
        }

        return cd;

    }

    fn interset(origin: Vec<usize>, other: Vec<usize>) -> Vec<usize> {
        let mut inter = Vec::new();
        for value in origin {
            if other.contains(&value) {
                inter.push(value);
            }
        }
        return inter;
    }

    let mut deck_size = HashMap::new();
    for i in 0..deck.len() {
        let count = deck_size.entry(deck[i]).or_insert(0);
        *count += 1;
    }
    let mut cd_set: Option<Vec<usize>> = Option::None;
    let mut last = 0;
    for value in deck_size.values() {

        if last != 0 {
            
            let cd = _cd(last, *value);
            if cd_set.is_some() {
                cd_set = Option::from(interset(cd_set.unwrap(), cd));
            } else {
                cd_set = Option::from(cd);
            }
        }
        last = *value;
    }
    return (cd_set.is_none() || cd_set.unwrap().len() > 0) && last > 1;

}