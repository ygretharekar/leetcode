#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut i = 0;

        let n = str.as_bytes();

        while i < n.len() && n[i] == b' ' {
            i += 1;
        }

        if i == str.len() {
            return 0;
        }

        let sign = if n[i] == b'-' {
            i += 1;
            true
         } else if n[i] == b'+' {
            i += 1;
            false
         } else if n[i].is_ascii_digit() {
             false
         } else {
             return 0;
         };

         let mut ans: i64 = 0;

         while i < n.len() && n[i].is_ascii_digit() {
            ans = ((n[i] - b'0') as i64) + ans * 10;
            i += 1;

            let max = std::i32::MAX as i64 + sign as i64;

            if ans > max {
                ans = max;
                break;
            }
        }

        if sign {
            -ans as i32
        }
        else {
            ans as i32
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::my_atoi(String::from("      -123 ffffff")),
            -123
        );
    }
}
