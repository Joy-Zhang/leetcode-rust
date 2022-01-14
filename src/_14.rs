pub fn longest_common_prefix(strs: Vec<String>) -> String {

    let mut min_len = 300;
    let borrowed_strs = &strs;
    for item in borrowed_strs {
        if item.len() < min_len {
            min_len = item.len();
        }
    } 

    let mut result = String::from("");

    for i in 0..min_len {
        
        let slice = borrowed_strs.first().unwrap().get(i..i+1);

        for o in borrowed_strs {
            if o.get(i..i+1) != slice {
                return result;
            }
        }
        result = result + slice.unwrap();
    }


    return result
}