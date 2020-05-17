#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {

        let sign = if x < 0 { -1 } else {1};

        let num = if x < 0 { x * -1 } else {x};

        let s = num.to_string();

        match s.chars().rev().collect::<String>().parse::<i32>() {
            Ok(ans) => ans * sign,
            Err(_) => 0
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reverse(1534236469),
            0
        );
    }
}
