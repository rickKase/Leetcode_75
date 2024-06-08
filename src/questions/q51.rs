/*
Question 51: N-Queens

Question:
---------
The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
Return all distinct solutions. Each solution contains a distinct board configuration of the n-queens' placement,
where 'Q' and '.' both indicate a queen and an empty space respectively.

Example:
---------
Input: n = 4
Output: [
 [".Q..",
  "...Q",
  "Q...",
  "..Q."],
 ["..Q.",
  "Q...",
  "...Q",
  ".Q.."]
]
*/

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut solutions = Vec::new();
    let mut board = vec![vec!['.'; n]; n];

    // Backtracking helper function.
    fn backtrack(
        row: usize,
        n: usize,
        board: &mut Vec<Vec<char>>,
        solutions: &mut Vec<Vec<String>>,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
    ) {
        if row == n {
            let solution = board.iter().map(|r| r.iter().collect()).collect();
            solutions.push(solution);
            return;
        }
        for col in 0..n {
            if cols[col] || diag1[row + col] || diag2[row + n - 1 - col] {
                continue;
            }
            // Place queen.
            cols[col] = true;
            diag1[row + col] = true;
            diag2[row + n - 1 - col] = true;
            board[row][col] = 'Q';

            backtrack(row + 1, n, board, solutions, cols, diag1, diag2);

            // Remove queen.
            board[row][col] = '.';
            cols[col] = false;
            diag1[row + col] = false;
            diag2[row + n - 1 - col] = false;
        }
    }

    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n - 1];
    let mut diag2 = vec![false; 2 * n - 1];
    backtrack(0, n, &mut board, &mut solutions, &mut cols, &mut diag1, &mut diag2);
    solutions
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 51: N-Queens");
    println!("---------------------");
    println!("Place n queens on an n×n chessboard so that no two queens attack each other.");
    println!("Return all distinct board configurations.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_n_queens() {
        let solutions = solve_n_queens(4);
        // There are 2 distinct solutions for n = 4.
        assert_eq!(solutions.len(), 2);
        // Each solution should have 4 rows.
        for sol in solutions {
            assert_eq!(sol.len(), 4);
        }
    }
}
