pub struct Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut max_right = -1;

        for num in arr.iter_mut().rev() {
            let current = *num;
            *num = max_right;
            max_right = max_right.max(current)
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::replace_elements(vec![17, 18, 3, 1, 2]),
            vec![18, 3, 2, 2, -1]
        )
    }
}
