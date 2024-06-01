/*
Question 32: Jump Game

Question:
---------
Given an array of non-negative integers `nums`, you are initially positioned at the first index of the array.
Each element in the array represents your maximum jump length at that position.
Determine if you can reach the last index.

Example:
---------
Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
*/

/// Returns true if you can reach the last index of the array.
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut furthest = 0;
    for (i, &num) in nums.iter().enumerate() {
        if i > furthest {
            return false;
        }
        furthest = furthest.max(i + num as usize);
        if furthest >= nums.len() - 1 {
            return true;
        }
    }
    true
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 32: Jump Game");
    println!("----------------------");
    println!("Determine if you can reach the last index of the array given jump lengths.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_can_jump_true() {
        assert_eq!(can_jump(vec![2,3,1,1,4]), true);
    }
    
    #[test]
    fn test_can_jump_false() {
        assert_eq!(can_jump(vec![3,2,1,0,4]), false);
    }
}
