#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        use std::str::Chars;

        let mut ans = String::new();
        let mut iters: Vec<Chars> = strs.iter().map(|s| {s.chars()}).collect();

        let mut last_char: Option<char> = None;

        if strs.len() == 0 {
            return ans;
        }

        loop {
            last_char.take().map(|ch| {ans.push(ch)});
            for i in iters.iter_mut() {
                let mut c = i.next();
                if c.is_none() {
                    return ans;
                }
                else {
                    match last_char {
                        None => last_char = c.take(),
                        Some(ch) => {
                            if ch != c.unwrap() {
                                return ans;
                            }
                        }
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::longest_common_prefix(vec![String::from("fruit"), String::from("france"),]),
            String::from("fr")
        );
    }
}
