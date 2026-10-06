use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counts: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            *counts.entry(c).or_insert(0) -= 1;
        }

        counts.values().all(|&v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_anagram(String::from("abc"), String::from("cab")),
            true
        );
    }
}
