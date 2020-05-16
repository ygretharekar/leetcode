#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut dict = vec![];
        dict.resize(num_rows as usize, vec![]);

        let m = if num_rows == 1 {1} else { 2*num_rows - 2 };

        for (i, c) in s.chars().enumerate() {
            let mut ind = i % m as usize;

            if ind >= num_rows as usize {
                ind = m as usize - ind;
            }

            dict[ind].push(c);
        }

        dict.iter().flatten().collect::<String>()
    }
}


#[cfg(test)]
mod tests6 {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
    }
}
