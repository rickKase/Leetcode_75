/*
Question 43: Longest Consecutive Sequence

Question:
---------
Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
Your algorithm should run in O(n) time.

Example:
---------
Input: [100, 4, 200, 1, 3, 2]
Output: 4
Explanation: The longest consecutive sequence is [1, 2, 3, 4].
*/

use std::collections::HashSet;

/// Returns the length of the longest consecutive sequence in the array.
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut longest = 0;
    for &num in set.iter() {
        if !set.contains(&(num - 1)) {
            let mut current = num;
            let mut length = 1;
            while set.contains(&(current + 1)) {
                current += 1;
                length += 1;
            }
            longest = longest.max(length);
        }
    }
    longest
}

pub fn print_question() {
    println!("Question 43: Longest Consecutive Sequence");
    println!("-----------------------------------------");
    println!("Find the length of the longest consecutive sequence in an unsorted array.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_consecutive() {
        assert_eq!(longest_consecutive(vec![100,4,200,1,3,2]), 4);
        assert_eq!(longest_consecutive(vec![]), 0);
    }
}
