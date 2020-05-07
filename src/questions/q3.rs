/*
Leetcode Problem 3: Contains Duplicate

Question:
---------
Given an integer array `nums`, return `true` if any value appears at least twice in the array,
and return `false` if every element is distinct.
*/

use std::collections::HashSet;

/// Returns true if any duplicate is found in the array.
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for num in nums {
        if !seen.insert(num) {
            return true;
        }
    }
    false
}

/// Prints the question to the CLI.
pub fn print_question() {
    println!("Leetcode Problem 3: Contains Duplicate");
    println!("----------------------------------------");
    println!("Given an integer array `nums`, return `true` if any value appears at least twice in the array,");
    println!("and return `false` if every element is distinct.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }
}
