#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut r = 0;
        let mut n = x;

        while n > r {
            r = r * 10 + n % 10;
            n /= 10;
        }

        n == r || n == r / 10
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_palindrome(1),
            true
        );
    }
}
