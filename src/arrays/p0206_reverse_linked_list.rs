pub struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = head;

        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in v.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    fn to_vec(mut cur: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = vec![];
        while let Some(node) = cur {
            out.push(node.val);
            cur = &node.next;
        }
        out
    }

    #[test]
    fn example_1() {
        let result = Solution::reverse_list(from_vec(vec![1, 2, 3, 4, 5]));
        assert_eq!(to_vec(&result), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn example_2() {
        let result = Solution::reverse_list(from_vec(vec![1, 2]));
        assert_eq!(to_vec(&result), vec![2, 1]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(Solution::reverse_list(None), None);
    }

    #[test]
    fn single_node() {
        let result = Solution::reverse_list(from_vec(vec![7]));
        assert_eq!(to_vec(&result), vec![7]);
    }
}
