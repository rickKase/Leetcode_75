/*
Question 34: Unique Binary Search Trees

Question:
---------
Given an integer n, return the number of structurally unique BST's (binary search trees) that store values 1 to n.
Example:
---------
Input: n = 3
Output: 5
Explanation: There are 5 unique BST's that can be constructed with nodes 1, 2, and 3.
*/

pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    if n == 0 { return 1; }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
         for j in 0..i {
              dp[i] += dp[j] * dp[i - j - 1];
         }
    }
    dp[n]
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 34: Unique Binary Search Trees");
    println!("---------------------------------------");
    println!("Return the number of unique BST's that can be constructed with n distinct nodes.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_num_trees() {
         assert_eq!(num_trees(3), 5);
         assert_eq!(num_trees(1), 1);
         // By convention, num_trees(0) is defined as 1.
         assert_eq!(num_trees(0), 1);
    }
}
