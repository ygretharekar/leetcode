#![allow(dead_code)]

pub struct Solution {}

enum State {
    Default,
    S1,
    S2
}

impl Solution {
    fn letter_to_int(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
             _  => 0
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut last_val: Option<i32> = None;
        let mut sum = 0;
        let mut current_state = State::Default;

        for c in s.chars().rev() {
            let val = Solution::letter_to_int(c);

            match current_state {
                State::Default => {
                    sum += val;
                    current_state = State::S1;
                },
                State::S1 => {
                    if val < last_val.unwrap() {
                        sum -= val;
                        current_state = State::S2;
                    } else {
                        sum += val;
                        current_state = State::S1;
                    }
                },
                State::S2 => {
                     sum += val;
                     current_state = State::S1;
                }
            }

            last_val.replace(val);
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
            Solution::roman_to_int(String::from("MCMXCIV")),
            1994
        );
    }
}
