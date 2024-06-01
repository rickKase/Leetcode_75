/*
Question 31: Maximum Product Subarray

Question:
---------
Given an integer array `nums`, find a contiguous non-empty subarray within the array that has the largest product, and return the product.
Example:
---------
Input: nums = [2,3,-2,4]
Output: 6
Explanation: The subarray [2,3] has the largest product 6.
*/

pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
         return 0;
    }
    let mut result = nums[0];
    let mut cur_max = nums[0];
    let mut cur_min = nums[0];
    for &num in nums.iter().skip(1) {
         let temp = cur_max; // store previous max
         cur_max = num.max(temp * num).max(cur_min * num);
         cur_min = num.min(temp * num).min(cur_min * num);
         result = result.max(cur_max);
    }
    result
}

pub fn print_question() {
    println!("Question 31: Maximum Product Subarray");
    println!("-------------------------------------");
    println!("Given an array of integers, return the largest product of a contiguous subarray.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_product() {
         assert_eq!(max_product(vec![2,3,-2,4]), 6);
    }
}
