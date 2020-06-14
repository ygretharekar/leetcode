#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {

        use std::collections::HashMap;
        
        let mut roman = HashMap::new();
    
        roman.insert( 1, "I");
        roman.insert( 5, "V");
        roman.insert( 10, "X");
        roman.insert( 50, "L");
        roman.insert( 100, "C");
        roman.insert( 500, "D");
        roman.insert( 1000, "M");

        let mut divider = 1000;

        let mut dividend = num;

        let mut ans = String::new();

        while divider > 0 {
            let mut d = dividend / divider;

            dividend = dividend % divider;

            if d == 0 {
                divider /= 10;
                continue;
            }
            
            if d < 5 {
                if d == 4 {
                    ans.push_str(roman[&divider]);
                    ans.push_str(roman[&(divider*5)]);
                } else {
                    while d > 0 {
                        ans.push_str(roman[&divider]);
                        d -= 1;
                    }
                }
            } else {
                if d == 9 {
                    ans.push_str(roman[&divider]);
                    ans.push_str(roman[&(divider*10)]);
                } else {
                    ans.push_str(roman[&(divider*5)]);

                    d -= 5;

                    while d > 0 {
                        ans.push_str(roman[&divider]);
                        d -= 1;
                    }
                }
            }

            divider /= 10;
        }

        ans
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::int_to_roman(1994),
            "MCMXCIV"
        );
    }
}
