pub struct Solution;

impl Solution {
    pub fn find_all_palindromes(s: String) {
        let s = s.as_bytes();
        let mut table = StateTable::new(s.len());
        for i in 0..s.len() {
            table.set(i, i, true);
            if i < s.len() - 1 {
                let is_palindromic = s[i] == s[i + 1];
                table.set(i, i + 1, is_palindromic)
            }
        }
        // the first two diagonals are filled in the loop above
        for diagonal in 2..s.len() {
            for i in 0..s.len() - diagonal {
                let j = i + diagonal;
                let is_palindromic = table.get(i + 1, j - 1) && s[i] == s[j];
                table.set(i, j, is_palindromic);
            }
        }
        for diagonal in (0..s.len()).rev() {
            for i in 0..s.len() - diagonal {
                let j = i + diagonal;
                if table.get(i, j) {
                    let str = unsafe { String::from_utf8_unchecked(Vec::from(&s[i..=j])) };
                    println!("{}", str);
                }
            }
        }
    }
}

struct StateTable {
    vec: Vec<u8>,
    size: usize,
}

impl StateTable {
    fn new(size: usize) -> StateTable {
        let mut vec = Vec::new();
        for _ in 0..(size * size + 7) / 8 {
            vec.push(0);
        }
        StateTable { vec, size }
    }

    fn get(&self, i: usize, j: usize) -> bool {
        self.vec[self.byte_index(i, j)] & 1 << self.bit_index(i, j) != 0
    }

    fn set(&mut self, i: usize, j: usize, value: bool) {
        let byte_index = self.byte_index(i, j);
        let bit_index = self.bit_index(i, j);
        if value {
            self.vec[byte_index] |= 1 << bit_index
        } else {
            self.vec[byte_index] &= !(1 << bit_index)
        }
    }

    fn byte_index(&self, i: usize, j: usize) -> usize {
        (i * self.size + j) / 8
    }

    fn bit_index(&self, i: usize, j: usize) -> usize {
        (i * self.size + j) % 8
    }
}
