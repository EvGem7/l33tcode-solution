use std::result;

use crate::problems::median_of_two_sorted_arrays::Solution;

mod problems;

fn main() {
    let n1 = vec![1, 2, 3];
    let n2 = vec![1, 2, 3, 4, 5,];
    let result = Solution::find_median_sorted_arrays(n1, n2);
    println!("result= {}", result);
}
