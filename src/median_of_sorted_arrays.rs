#![allow(dead_code)]

use std::cmp::Ordering;

pub struct Solution {}

fn binary_seach_median<T, F>(a: &[T], mut f: F) -> Result<usize, usize>
where 
    F: FnMut(usize) -> Ordering,
{
    let (mut left, mut right) = (0, a.len());

    while right > left {
        let mid = (left + right) >> 1;

        match f(mid) {
            Ordering::Less => right = mid,
            Ordering::Greater => left = mid + 1,
            Ordering::Equal => {return Ok(mid);},
        }
    }

    Err(left)
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());

        let (a, b, m, n) = if m > n {
            (&nums2, &nums1, n, m)
        } else {
            (&nums1, &nums2, m, n)
        };

        let mid = (m + n + 1) >> 1;

        binary_seach_median(
            a, 
            |i| {

                let j = mid - i - 1;

                if a[i] > b[j] {
                    Ordering::Less
                }
                else if i + 1 < m  && j > 1 && b[j - 1] > a[i + 1] {
                    Ordering::Greater
                }
                else {
                    Ordering::Equal
                }
            }
        )
        .map(|i| i + 1)
        .or_else(|i| -> Result<usize, usize> {Ok(i)})
        .map(|i| {

            let j = mid - i;

            let l_max = if i == 0 {
                b[j - 1]
            } else if j == 0 {
                a[i - 1]
            } else {
                std::cmp::max(a[i - 1], b[j - 1])
            };

            if (m + n) % 2 == 1 {
                l_max as f64
            }
            else {
                let r_min = if i == m {
                    b[j]
                }
                else if j == n {
                    a[i]
                }
                else {
                    std::cmp::min(a[i], b[j])
                };

                (l_max + r_min) as f64 / 2.0
            }
        })
        .unwrap()
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
