/*
Question 52: N-Queens II

Question:
---------
The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
Return the total number of distinct solutions.

Example:
---------
Input: n = 4
Output: 2
*/

pub fn total_n_queens(n: i32) -> i32 {
    let n = n as usize;
    let mut count = 0;

    fn backtrack(
        row: usize,
        n: usize,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        count: &mut i32,
    ) {
        if row == n {
            *count += 1;
            return;
        }
        for col in 0..n {
            if cols[col] || diag1[row + col] || diag2[row + n - 1 - col] {
                continue;
            }
            cols[col] = true;
            diag1[row + col] = true;
            diag2[row + n - 1 - col] = true;

            backtrack(row + 1, n, cols, diag1, diag2, count);

            cols[col] = false;
            diag1[row + col] = false;
            diag2[row + n - 1 - col] = false;
        }
    }

    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n - 1];
    let mut diag2 = vec![false; 2 * n - 1];
    backtrack(0, n, &mut cols, &mut diag1, &mut diag2, &mut count);
    count
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 52: N-Queens II");
    println!("------------------------");
    println!("Count the number of distinct solutions to the n-queens puzzle.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_n_queens() {
        assert_eq!(total_n_queens(4), 2);
        assert_eq!(total_n_queens(1), 1);
    }
}
