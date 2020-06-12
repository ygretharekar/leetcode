#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let dec = | x: &mut usize| {
            let t = *x;
            *x -= 1;
            t
        };

        let inc = | x: &mut usize| {
            let t = *x;
            *x += 1;
            t
        };

        let (mut l, mut r) = (0, height.len() - 1);

        let mut max = 0;

        while l < r {
            let w = (r - l) as i32;
            
            let h = if height[l] > height[r] {
                height[dec(&mut r)]
            } else {
                height[inc(&mut l)]
            };
            max = std::cmp::max(max, w * h);
        }

        max
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_area(vec![1,8,6,2,5,4,8,3,7]),
            49
        );
    }
}
