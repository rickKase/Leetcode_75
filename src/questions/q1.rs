/*
Leetcode Problem 1: Two Sum

Question:
---------
Given an array of integers `nums` and an integer `target`, return the indices of the two numbers such that they add up to `target`.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.
*/

use std::collections::HashMap;

/// Returns the indices of the two numbers that add up to the target.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_to_index = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = num_to_index.get(&complement) {
            return vec![j as i32, i as i32];
        }
        num_to_index.insert(num, i);
    }
    vec![]
}

/// Prints the question to the CLI.
pub fn print_question() {
    println!("Leetcode Problem 1: Two Sum");
    println!("-----------------------------");
    println!("Given an array of integers `nums` and an integer `target`,");
    println!("return the indices of the two numbers such that they add up to `target`.");
    println!("You may assume that each input would have exactly one solution,");
    println!("and you may not use the same element twice.");
    println!("You can return the answer in any order.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums.clone(), target);
        // Check that we received exactly two indices.
        assert_eq!(result.len(), 2);
        // Verify that the numbers at the returned indices sum to the target.
        assert_eq!(nums[result[0] as usize] + nums[result[1] as usize], target);
    }
}
