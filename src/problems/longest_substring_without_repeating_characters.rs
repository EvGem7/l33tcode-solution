use std::cmp::min;
use std::collections::HashSet;

pub struct Solution;

impl Solution {

    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut set = HashSet::new();
        for byte in bytes {
            set.insert(*byte);
        }
        let alphabet_size = set.len();

        let mut max_cnt = 0;
        for i in 0..bytes.len() {
            set.clear();
            for j in i..bytes.len() {
                let byte = bytes[j];
                if set.contains(&byte) {
                    break;
                } else {
                    set.insert(byte);
                }
            }
            if max_cnt < set.len() {
                max_cnt = set.len();
            }
            if max_cnt >= alphabet_size {
                break
            }
        }
        return max_cnt as i32;
    }
}