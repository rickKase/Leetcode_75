/*
Question 22: Minimum Path Sum

Question:
---------
Given a m x n grid filled with non-negative numbers, find a path from top-left to bottom-right which minimizes the sum of all numbers along its path.
You can only move either down or right at any point in time.

Example:
---------
Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
Output: 7
Explanation: Because the path 1→3→1→1→1 minimizes the sum.
*/

/// Returns the minimum path sum from the top-left to bottom-right of a grid.
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![0; n]; m];
    
    dp[0][0] = grid[0][0];
    // Initialize first row.
    for j in 1..n {
        dp[0][j] = dp[0][j - 1] + grid[0][j];
    }
    // Initialize first column.
    for i in 1..m {
        dp[i][0] = dp[i - 1][0] + grid[i][0];
    }
    
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = grid[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
        }
    }
    dp[m - 1][n - 1]
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 22: Minimum Path Sum");
    println!("-----------------------------");
    println!("Find the minimum path sum in a grid from top-left to bottom-right, moving only down or right.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1]
        ];
        assert_eq!(min_path_sum(grid), 7);
    }
}
