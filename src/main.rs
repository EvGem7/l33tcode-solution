use std::result;

use crate::problems::median_of_two_sorted_arrays::Solution;

mod problems;

fn main() {
    let n1 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 4];
    let n2 = vec![1, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4];
    let result = Solution::find_median_sorted_arrays(n1, n2);
    println!("result= {}", result);
}
