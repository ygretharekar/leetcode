#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let st = s.as_bytes();

        let mut max: usize;

        let mut longest = &st[0..0];

        for i in 0..s.len() {
            for j in i..i + 2 {
                let (mut b, mut e) = (i, j);
                while e < st.len() {
                    if st[b] == st[e] {
                        max = e - b + 1;

                        if max > longest.len() {
                            longest = &st[b..e + 1];
                        }
                    }
                    else {
                        break;
                    }
                    if b == 0 {
                        break;
                    }

                    b -= 1;
                    e += 1; 

                }
            }
        }

        String::from_utf8(longest.to_vec()).unwrap()
    }    
}


#[cfg(test)]
mod tests5 {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::longest_palindrome(String::from("cbbd")),
            String::from("bb")
        );
    }
}
