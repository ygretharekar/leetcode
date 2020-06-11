#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        let mut matched = vec![vec![false; p.len() + 1]; 2];

        let (mut old, mut new) = (1, 0);

        for i in (0.. s.len() + 1).rev() {
            for j in (0..p.len() + 1).rev() {
                let (t, p) = (&s[i..], &p[j..]);

                matched[new][j] = if p.is_empty() {
                    t.is_empty()
                } else {

                    let first_match = !t.is_empty() && 
                        (t.as_bytes()[0] == p.as_bytes()[0] || p.as_bytes()[0] == b'.');

                    if p.len() >= 2 && p.as_bytes()[1] == b'*' {
                        (first_match && matched[old][j]) || matched[new][j + 2]
                    } else {
                        first_match && matched[old][j + 1]
                    }                    
                }
            }
            std::mem::swap(&mut new,  &mut old)
        }
        matched[old][0]
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_match(String::from("aaaaaab"), String::from("a*b")),
            true
        );
    }
}
