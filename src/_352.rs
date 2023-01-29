use std::collections::BTreeMap;

pub struct SummaryRanges {
    storage: BTreeMap<i32, i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {

    pub fn new() -> Self {
        return SummaryRanges {
            storage: BTreeMap::new()
        }
    }
    
    pub fn add_num(&mut self, value: i32) {

        fn _keys(s: &mut BTreeMap<i32, i32>) -> Vec<i32> {
            let mut vec = Vec::new();
            for key in s.keys() {
                vec.push(*key);
            }
            return vec;
        }

        let _storage = &mut self.storage;
        let keys = _keys(_storage);
        
        
        let mut updated_key_ref = Option::None;
        let mut insert = true;
        for key in &keys {
        
            let length = _storage.get(key).copied().unwrap();
            if key + length == value {
                _storage.insert(*key, length + 1);
                updated_key_ref = Option::from(key);
                insert = false;
            }
            if value + 1 == *key {
                if updated_key_ref.is_none() {
                    _storage.insert(value, length + 1);
                } else {
                    let updated_key = updated_key_ref.copied().unwrap();
                    _storage.entry(updated_key).and_modify(|l| { *l += length; });
                }
                _storage.remove(key);
                insert = false;
            }
            if *key > value {
                break;
            }
        }
        if insert {
            _storage.insert(value, 1);
        }
    }
    
    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for key in self.storage.keys() {
            result.push(vec!(*key, key + self.storage.get(key).copied().unwrap() - 1))
        }
        return result;
    }
}