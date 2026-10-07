pub struct Solution;

// Given an integer array nums of length n, you want to create an array ans of length 2n
// where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).
//
// Specifically, ans is the concatenation of two nums arrays.
//
// Return the array ans.
//
//
//
// Example 1:
//
// Input: nums = [1,2,1]
//
// Output: [1,2,1,1,2,1]
//
// Explanation: The array ans is formed as follows:
// - ans = [nums[0],nums[1],nums[2],nums[0],nums[1],nums[2]]
// - ans = [1,2,1,1,2,1]
//
// Example 2:
//
// Input: nums = [1,3,2,1]
//
// Output: [1,3,2,1,1,3,2,1]
//
// Explanation: The array ans is formed as follows:
// - ans = [nums[0],nums[1],nums[2],nums[3],nums[0],nums[1],nums[2],nums[3]]
// - ans = [1,3,2,1,1,3,2,1]
//
//
//
// Constraints:
//
// n == nums.length
// 1 <= n <= 1000
// 1 <= nums[i] <= 1000

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::from(nums);

        v.extend_from_within(..);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        )
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::get_concatenation(vec![7]), vec![7, 7])
    }

    #[test]
    fn length_is_doubled() {
        let nums: Vec<i32> = (1..=1000).collect();
        let ans = Solution::get_concatenation(nums.clone());
        assert_eq!(ans.len(), 2 * nums.len());
        assert_eq!(&ans[..nums.len()], &nums[..]);
        assert_eq!(&ans[nums.len()..], &nums[..]);
    }
}
