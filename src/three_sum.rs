#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();

        use std::collections::HashSet;

        let mut ans: HashSet<Vec<i32>> = HashSet::new();

        nums.sort();

        for (i, x) in nums.iter().enumerate() {
            if *x > 0 {
                break;
            }

            let sum = -x;

            let mut found: HashSet<i32> = HashSet::new();

            for y in nums[i+1..].iter() {
                if found.contains(y) {
                    ans.insert(vec![*x, sum - y, *y]);
                } else {
                   found.insert(sum - y); 
                }
            }
        }

        ans.into_iter().collect()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::three_sum(vec![-1, 1, 0]),
            vec![vec![-1, 0, 1]]
        );
    }
}
