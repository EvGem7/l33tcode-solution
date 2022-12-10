use std::cmp::{min, Ordering};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, write};
use std::ops::Add;
use std::ptr::null;

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let cd = CommonData {
            final_arr_size: nums1.len() + nums2.len(),
        };
        let mut finder1 = &mut Finder::new(nums1, cd);
        let mut finder2 = &mut Finder::new(nums2, cd);
        Solution::check_candidates(finder1, finder2)
            .or_else(|| Solution::check_candidates(finder2, finder1))
            .unwrap()
    }

    fn check_candidates(primary: &mut Finder, secondary: &mut Finder) -> Option<f64> {
        // println!("check_candidates {:?}, {:?}", primary.nums, secondary.nums);
        let mut candidate;
        while {
            candidate = primary.next();
            candidate.is_some()
        } {
            let candidate = &candidate.unwrap();
            // println!("{:?}", candidate);
            let result = &secondary.check(candidate);
            primary.put_check_result(candidate, result);
            match primary.get_median() {
                None => continue,
                Some(median) => return Some(median),
            }
        }
        None
    }
}

#[derive(Copy, Clone)]
struct CommonData {
    pub final_arr_size: usize,
}

impl CommonData {
    fn before_l_median_size(&self) -> usize { (self.final_arr_size - 1) / 2 }
    fn before_r_median_size(&self) -> usize { (self.final_arr_size - 0) / 2 }
}

#[derive(Copy, Clone, Debug)]
struct Indices {
    start: usize,
    end_exclusive: usize,
}

impl Indices {
    fn new(start: usize, end_exclusive: usize) -> Indices {
        Indices { start, end_exclusive }
    }
}

impl Display for Indices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.start, self.end_exclusive)
    }
}

#[derive(Debug)]
struct Candidate {
    num: i32,
    source_indices: Indices,
}

impl Candidate {
    fn new(num: i32, source_indices: Indices) -> Candidate {
        Candidate { num, source_indices }
    }
}

struct CheckResult {
    indices: Indices,
}

impl CheckResult {
    fn new(indices: Indices) -> CheckResult {
        CheckResult { indices }
    }
}

struct Finder {
    nums: Vec<i32>,
    cd: CommonData,
    l: usize,
    r: usize,
    map: HashMap<i32, NumStats>,
    l_median: Option<i32>,
    r_median: Option<i32>,
}

impl Finder {
    fn new(nums: Vec<i32>, cd: CommonData) -> Finder {
        Finder {
            l: 0,
            r: min(cd.before_r_median_size() + 1, nums.len()),
            nums,
            cd,
            map: HashMap::new(),
            l_median: None,
            r_median: None,
        }
    }

    fn next(&mut self) -> Option<Candidate> {
        while self.l < self.r {
            // println!("next l={}, r={}", self.l, self.r);
            let m = (self.r - self.l) / 2 + self.l;
            let num = self.nums[m];
            if self.map.contains_key(&num) {
                let num_stats = self.map[&num];
                let indices = num_stats.source_indices;
                if self.cd.before_l_median_size() < num_stats.min_nums_before {
                    self.r = indices.start;
                } else if self.cd.before_r_median_size() > num_stats.max_nums_before {
                    self.l = indices.end_exclusive;
                } else {
                    panic!("next found median!");
                }
            } else {
                return Some(Candidate::new(num, self.find_indices(num)));
            }
        }
        // println!("loop end l={}, r={}", self.l, self.r);
        None
    }

    fn find_indices(&self, num: i32) -> Indices {
        let start = self.nums.partition_point(|&x| x < num);
        let end = self.nums.partition_point(|&x| x <= num);
        Indices::new(start, end)
    }

    fn check(&mut self, candidate: &Candidate) -> CheckResult {
        let indices = self.find_indices(candidate.num);
        let result = CheckResult::new(indices);
        self.put_check_result(candidate, &result);
        self.map.insert(candidate.num, NumStats {
            source_indices: indices,
            ..self.map[&candidate.num]
        });
        result
    }

    fn put_check_result(&mut self, candidate: &Candidate, result: &CheckResult) {
        let source_indices = candidate.source_indices;
        let num_stats = NumStats {
            source_indices: candidate.source_indices,
            min_nums_before: source_indices.start + result.indices.start,
            max_nums_before: source_indices.end_exclusive + result.indices.end_exclusive - 1,
        };
        // println!("{:?} for {:?}", num_stats, candidate);
        self.map.insert(candidate.num, num_stats);
        if num_stats.contains(self.cd.before_l_median_size()) {
            // println!("found l_median: {}", candidate.num);
            self.l_median = Some(candidate.num);
        }
        if num_stats.contains(self.cd.before_r_median_size()) {
            // println!("found r_median: {}", candidate.num);
            self.r_median = Some(candidate.num);
        }
    }

    fn get_median(&self) -> Option<f64> {
        self.l_median
            .zip(self.r_median)
            .map(|(l, r)| (l + r) as f64 / 2f64)
    }
}

#[derive(Copy, Clone, Debug)]
struct NumStats {
    source_indices: Indices,
    min_nums_before: usize,
    max_nums_before: usize,
}

impl NumStats {
    fn contains(&self, size: usize) -> bool {
        size >= self.min_nums_before && size <= self.max_nums_before
    }
}
