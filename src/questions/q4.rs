/*
Leetcode Problem 4: Product of Array Except Self

Question:
---------
Given an integer array `nums`, return an array `answer` such that `answer[i]` is equal to the product of all the elements of `nums` except `nums[i]`.
The product of any prefix or suffix of `nums` is guaranteed to fit in a 32-bit integer.
You must write an algorithm that runs in O(n) time and without using the division operation.
*/

/// Returns a vector where each element is the product of every other element in the input.
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut answer = vec![1; n];

    // Compute left cumulative products.
    for i in 1..n {
        answer[i] = answer[i - 1] * nums[i - 1];
    }
    
    // Compute right cumulative products and multiply with left products.
    let mut right = 1;
    for i in (0..n).rev() {
        answer[i] *= right;
        right *= nums[i];
    }
    
    answer
}

/// Prints the question to the CLI.
pub fn print_question() {
    println!("Leetcode Problem 4: Product of Array Except Self");
    println!("-------------------------------------------------");
    println!("Given an integer array `nums`, return an array `answer` such that `answer[i]` is equal to the product");
    println!("of all the elements of `nums` except `nums[i]`.");
    println!("The product of any prefix or suffix of `nums` is guaranteed to fit in a 32-bit integer.");
    println!("You must write an algorithm that runs in O(n) time and without using the division operation.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}
