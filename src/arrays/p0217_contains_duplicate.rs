use std::collections::HashSet;

pub struct Solution;

// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,1]
//
// Output: true
//
// Explanation:
//
// The element 1 occurs at the indices 0 and 3.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
//
// Output: false
//
// Explanation:
//
// All elements are distinct.
//
// Example 3:
//
// Input: nums = [1,1,1,3,3,4,3,2,4,2]
//
// Output: true
//
//
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// -10^9 <= nums[i] <= 10^9

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut v: HashSet<i32> = HashSet::new();

        for num in nums {
            if !v.insert(num) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false)
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        )
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::contains_duplicate(vec![5]), false)
    }

    #[test]
    fn negatives_and_extremes() {
        assert_eq!(
            Solution::contains_duplicate(vec![-1_000_000_000, 1_000_000_000, -1_000_000_000]),
            true
        )
    }

    #[test]
    fn duplicate_at_start() {
        assert_eq!(Solution::contains_duplicate(vec![1, 1, 2]), true)
    }

    #[test]
    fn duplicate_at_end() {
        let mut nums: Vec<i32> = (0..100_000).collect();
        assert_eq!(Solution::contains_duplicate(nums.clone()), false);

        nums.push(99_999);
        assert_eq!(Solution::contains_duplicate(nums), true);
    }
}
