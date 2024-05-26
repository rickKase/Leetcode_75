/*
Question 21: Unique Paths

Question:
---------
A robot is located at the top-left corner of an m x n grid (marked 'Start' in the diagram).
The robot can only move either down or right at any point in time.
The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram).
How many possible unique paths are there?

Example:
---------
Input: m = 3, n = 7
Output: 28
*/

/// Returns the number of unique paths from the top-left to the bottom-right of an m x n grid.
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![1; n]; m];

    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[m - 1][n - 1]
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 21: Unique Paths");
    println!("-------------------------");
    println!("Count the number of unique paths in an m x n grid from top-left to bottom-right.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 7), 28);
        assert_eq!(unique_paths(3, 2), 3);
        assert_eq!(unique_paths(1, 1), 1);
    }
}
