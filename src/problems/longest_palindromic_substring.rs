use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut table: HashMap<(usize, usize), bool> = HashMap::new();
        for i in 0..s.len() {
            table.insert((i, i), true);
            if i < s.len() - 1 {
                let is_palindromic = s[i] == s[i + 1];
                table.insert((i, i + 1), is_palindromic);
            }
        }

        for diagonal in (0..s.len()).rev() {
            for i in 0..s.len() - diagonal {
                let j = i + diagonal;
                if Solution::is_palindrome(s, &mut table, i, j) {
                    return unsafe { String::from_utf8_unchecked(Vec::from(&s[i..=j])) };
                }
            }
        }

        // // the first two diagonals are filled in the loop above
        // for diagonal in 2..s.len() {
        //     for i in 0..s.len() - diagonal {
        //         let j = i + diagonal;
        //         let is_palindromic = table.get(i + 1, j - 1) && s[i] == s[j];
        //         table.set(i, j, is_palindromic);
        //     }
        // }
        // for diagonal in (0..s.len()).rev() {
        //     for i in 0..s.len() - diagonal {
        //         let j = i + diagonal;
        //         if table.get(i, j) {
        //             let str = unsafe { String::from_utf8_unchecked(Vec::from(&s[i..=j])) };
        //             println!("{}", str);
        //         }
        //     }
        // }
        "".to_string()
    }

    fn is_palindrome(s: &[u8], table: &mut HashMap<(usize, usize), bool>, i: usize, j: usize) -> bool {
        if let Some(&is_palindrome) = table.get(&(i, j)) {
            return is_palindrome;
        }
        let is_inner_palindrome = match table.get(&(i + 1, j - 1)) {
            None => Solution::is_palindrome(s, table, i + 1, j - 1),
            Some(&is_palindrome) => is_palindrome,
        };
        let is_palindrome = is_inner_palindrome && s[i] == s[j];
        table.insert((i, j), is_palindrome);
        is_palindrome
    }
}
