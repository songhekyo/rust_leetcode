use std::collections::HashMap;

pub struct Solution;

// Given an array of integers nums and an integer target, return indices of the two numbers
// such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may not use the
// same element twice.
//
// You can return the answer in any order.
//
//
//
// Example 1:
//
// Input: nums = [2,7,11,15], target = 9
//
// Output: [0,1]
//
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
// Example 2:
//
// Input: nums = [3,2,4], target = 6
//
// Output: [1,2]
//
// Example 3:
//
// Input: nums = [3,3], target = 6
//
// Output: [0,1]
//
//
//
// Constraints:
//
// 2 <= nums.length <= 10^4
// -10^9 <= nums[i] <= 10^9
// -10^9 <= target <= 10^9
// Only one valid answer exists.
//
//
// Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&j) = seen.get(&complement) {
                return vec![j as i32, i as i32];
            }

            seen.insert(num, i);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The answer may be returned in any order, so sort it before comparing.
    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    #[test]
    fn example_1() {
        assert_eq!(sorted(Solution::two_sum(vec![2, 7, 11, 15], 9)), vec![0, 1])
    }

    #[test]
    fn example_2() {
        assert_eq!(sorted(Solution::two_sum(vec![3, 2, 4], 6)), vec![1, 2])
    }

    #[test]
    fn example_3() {
        assert_eq!(sorted(Solution::two_sum(vec![3, 3], 6)), vec![0, 1])
    }

    #[test]
    fn same_element_not_used_twice() {
        // 3 + 3 == 6, but there is only one 3, so it must be 2 + 4
        assert_eq!(sorted(Solution::two_sum(vec![3, 2, 4], 6)), vec![1, 2]);
        assert_eq!(sorted(Solution::two_sum(vec![1, 5, 3, 7], 10)), vec![2, 3])
    }

    #[test]
    fn negatives() {
        assert_eq!(sorted(Solution::two_sum(vec![-3, 4, 3, 90], 0)), vec![0, 2])
    }

    #[test]
    fn pair_far_apart() {
        let mut nums: Vec<i32> = (1..=10_000).map(|x| x * 10).collect();
        nums[0] = 1;
        nums[9_999] = 2;
        assert_eq!(sorted(Solution::two_sum(nums, 3)), vec![0, 9_999])
    }
}
