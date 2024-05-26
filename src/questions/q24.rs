/*
Question 24: Longest Increasing Subsequence

Question:
---------
Given an integer array nums, return the length of the longest strictly increasing subsequence.
A subsequence is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements.

Example:
---------
Input: nums = [10,9,2,5,3,7,101,18]
Output: 4
Explanation: The longest increasing subsequence is [2,3,7,101], so the length is 4.
*/

/// Returns the length of the longest increasing subsequence in the given array.
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let n = nums.len();
    let mut dp = vec![1; n];
    for i in 0..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 24: Longest Increasing Subsequence");
    println!("---------------------------------------------");
    println!("Find the length of the longest strictly increasing subsequence in an array.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(length_of_lis(nums), 4);
        assert_eq!(length_of_lis(vec![]), 0);
        assert_eq!(length_of_lis(vec![1, 2, 3, 4, 5]), 5);
    }
}
