pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.is_empty() {
            return find_median_simple(&nums2);
        }
        if nums2.is_empty() {
            return find_median_simple(&nums1);
        }
        let mut l1: usize = 0;
        let mut r1: usize = nums1.len() - 1;
        let mut l2: usize = 0;
        let mut r2: usize = nums2.len() - 1;
        while l1 < r1 && l2 < r2 {
            println!("1:[{}..{}] 2:[{}..{}]", l1, r1, l2, r2);
            let m11 = (r1 - l1 + 0) / 2 + l1;
            let m12 = (r1 - l1 + 1) / 2 + l1;
            let m21 = (r2 - l2 + 0) / 2 + l2;
            let m22 = (r2 - l2 + 1) / 2 + l2;
            let avg1 = (nums1[m11] + nums1[m12]) as f64 / 2f64;
            let avg2 = (nums2[m21] + nums2[m22]) as f64 / 2f64;
            if nums1[m11] >= nums2[m21] && nums1[m12] <= nums2[m22] {
                return avg1;
            } else if nums2[m21] >= nums1[m11] && nums2[m22] <= nums1[m12] {
                return avg2;
            } else if avg1 > avg2 {
                r1 = if r1 - l1 == 1 { m11 } else { m12 };
                l2 = if r2 - l2 == 1 { m22 } else { m21 };
            } else {
                l1 = if r1 - l1 == 1 { m12 } else { m11 };
                r2 = if r2 - l2 == 1 { m21 } else { m22 };
            }
        }
        println!("while end 1:[{}..{}] 2:[{}..{}]", l1, r1, l2, r2);
        if l1 < r1 {
            find_median_with_single(&nums1[l1..=r1], nums2[l2])
        } else if l2 < r2 {
            find_median_with_single(&nums2[l2..=r2], nums1[l1])
        } else {
            (nums1[l1] + nums2[l2]) as f64 / 2f64
        }
    }
}

fn find_median_simple(nums: &[i32]) -> f64 {
    let m1 = (nums.len() - 1 + 0) / 2;
    let m2 = (nums.len() - 1 + 1) / 2;
    return (nums[m1] + nums[m2]) as f64 / 2f64;
}

fn find_median_with_single(nums: &[i32], single: i32) -> f64 {
    let i = match nums.binary_search(&single) {
        Ok(i) => i,
        Err(i) => i,
    };
    // minus 1 and plus 1 because of inserted [single]
    let m1 = (nums.len() + 0) / 2;
    let m2 = (nums.len() + 1) / 2;
    let get_num = |m: usize| if m != i {
        if m < i { nums[m] } else { nums[m - 1] }
    } else { single };
    (get_num(m1) + get_num(m2)) as f64 / 2f64
}