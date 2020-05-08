/*
Leetcode Problem 5: Maximum Subarray

Question:
---------
Given an integer array `nums`, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
*/

/// Returns the sum of the contiguous subarray with the largest sum.
pub fn max_subarray(nums: Vec<i32>) -> i32 {
    // Assume nums is non-empty.
    let mut current_sum = nums[0];
    let mut max_sum = nums[0];
    for &num in nums.iter().skip(1) {
        current_sum = (current_sum + num).max(num);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

/// Prints the question to the CLI.
pub fn print_question() {
    println!("Leetcode Problem 5: Maximum Subarray");
    println!("-------------------------------------");
    println!("Given an integer array `nums`, find the contiguous subarray (containing at least one number)");
    println!("which has the largest sum and return its sum.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_subarray() {
        assert_eq!(max_subarray(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
}
