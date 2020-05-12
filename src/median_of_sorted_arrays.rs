#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut m, mut n) = (nums1.len(), nums2.len());
        0.0
    }
}


#[cfg(test)]
mod tests4 {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1], vec![2]),
            0.0
        );
    }
}
