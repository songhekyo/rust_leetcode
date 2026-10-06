mod arrays;

use crate::arrays::{
    p0217_contains_duplicate, p0242_valid_anagram, p0485_max_consecutive_ones,
    p1299_replace_element_with_greatest_on_right_side, p1929_concetanation_array,
};

fn main() {
    print!(
        "{:?}",
        p0217_contains_duplicate::Solution::contains_duplicate(vec![1, 2, 3, 1])
    );

    print!(
        "{:?}",
        p0242_valid_anagram::Solution::is_anagram(String::from("anagram"), String::from("nagaram"))
    );

    print!(
        "{:?}",
        p0485_max_consecutive_ones::Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
    );

    print!(
        "{:?}",
        p1299_replace_element_with_greatest_on_right_side::Solution::replace_elements(vec![
            17, 18, 5, 4, 6, 1
        ])
    );

    print!(
        "{:?}",
        p1929_concetanation_array::Solution::get_concatenation(vec![1, 2, 1])
    )
}
