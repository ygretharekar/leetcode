#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        
        if nums.len() <= 3 {
            return nums.iter().sum();
        }
        
        let mut nums = nums.clone();

        nums.sort();

        let mut sum = nums[0] + nums[1] + nums[2];

        let mut closest = (target - sum).abs();

        for a in 0..nums.len() - 2 {
            let mut b = nums.len() - 1;
            let mut c = a + 1;

            while b > c {
                let s = nums[a] + nums[b] + nums[c];
                
                let comp = target - s;

                if comp.abs() < closest {
                    sum = s;
                    closest = comp.abs();
                }

                if comp < 0 {
                    b = b - 1;
                } else if comp > 0 {
                    c = c + 1;
                } else {
                    return sum;
                }
            }
        }
        
        sum
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::three_sum_closest(vec![-1, 2, 1, 4], 1),
            2
        );
        assert_eq!(
            Solution::three_sum_closest(vec![-1, 2, 1, 0], 0),
            0
        );
        assert_eq!(
            Solution::three_sum_closest(vec![0, 2, 1, -3], 1),
            0
        );
    }
}
