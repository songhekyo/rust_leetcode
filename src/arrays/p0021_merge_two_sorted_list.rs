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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let (Some(a), Some(b)) = (&list1, &list2) {
            let src = if a.val <= b.val {
                &mut list1
            } else {
                &mut list2
            };

            let mut node = src.take().unwrap();
            *src = node.next.take();
            tail = tail.next.insert(node);
        }
        tail.next = list1.or(list2);

        dummy.next
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
        let result = Solution::merge_two_lists(from_vec(vec![1, 2, 4]), from_vec(vec![1, 3, 4]));
        assert_eq!(to_vec(&result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn example_3() {
        let result = Solution::merge_two_lists(None, from_vec(vec![0]));
        assert_eq!(to_vec(&result), vec![0]);
    }
}
