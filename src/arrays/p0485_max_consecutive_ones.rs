pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut max = 0;

        for n in nums {
            if n == 1 {
                current += 1;

                if current > max {
                    max = current
                }
            } else {
                current = 0;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        )
    }
}
