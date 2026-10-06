mod arrays;

use arrays::p0217_contains_duplicate;

use crate::arrays::{p0242_valid_anagram, p0485_max_consecutive_ones};

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
    )
}
