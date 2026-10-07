mod arrays;

use crate::arrays::{
    p0021_merge_two_sorted_list, p0206_reverse_linked_list, p0217_contains_duplicate,
    p0242_valid_anagram, p0485_max_consecutive_ones,
    p1299_replace_element_with_greatest_on_right_side, p1929_concetanation_array,
};

fn main() {
    println!(
        "{:?}",
        p0217_contains_duplicate::Solution::contains_duplicate(vec![1, 2, 3, 1])
    );

    println!(
        "{:?}",
        p0242_valid_anagram::Solution::is_anagram(String::from("anagram"), String::from("nagaram"))
    );

    println!(
        "{:?}",
        p0485_max_consecutive_ones::Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
    );

    println!(
        "{:?}",
        p1299_replace_element_with_greatest_on_right_side::Solution::replace_elements(vec![
            17, 18, 5, 4, 6, 1
        ])
    );

    println!(
        "{:?}",
        p1929_concetanation_array::Solution::get_concatenation(vec![1, 2, 1])
    );

    {
        use p0206_reverse_linked_list::ListNode;

        // 1 -> 2
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        println!(
            "{:?}",
            p0206_reverse_linked_list::Solution::reverse_list(head)
        );
    }

    {
        use p0021_merge_two_sorted_list::ListNode;

        // 1 -> 3
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        }));
        // 2
        let list2 = Some(Box::new(ListNode { val: 2, next: None }));

        println!(
            "{:?}",
            p0021_merge_two_sorted_list::Solution::merge_two_lists(list1, list2)
        );
    }
}
