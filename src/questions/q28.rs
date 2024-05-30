/*
Question 28: Word Search

Question:
---------
Given an m x n grid of characters board and a string word, return true if word exists in the grid.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring.
The same letter cell may not be used more than once.

Example:
---------
Input: board = 
[
  ['A','B','C','E'],
  ['S','F','C','S'],
  ['A','D','E','E']
], word = "ABCCED"
Output: true
*/

/// Returns true if the given word exists in the board.
pub fn word_search(board: Vec<Vec<char>>, word: String) -> bool {
    if board.is_empty() || board[0].is_empty() {
        return false;
    }
    let rows = board.len();
    let cols = board[0].len();
    let word_chars: Vec<char> = word.chars().collect();

    fn dfs(
        board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        i: usize,
        j: usize,
        index: usize,
        rows: usize,
        cols: usize,
    ) -> bool {
        if index == word.len() {
            return true;
        }
        if i >= rows || j >= cols || board[i][j] != word[index] {
            return false;
        }
        // Temporarily mark the cell as visited.
        let temp = board[i][j];
        board[i][j] = '#';

        // Explore neighbors.
        let found = (if i > 0 { dfs(board, word, i - 1, j, index + 1, rows, cols) } else { false }) ||
                    (if i + 1 < rows { dfs(board, word, i + 1, j, index + 1, rows, cols) } else { false }) ||
                    (if j > 0 { dfs(board, word, i, j - 1, index + 1, rows, cols) } else { false }) ||
                    (if j + 1 < cols { dfs(board, word, i, j + 1, index + 1, rows, cols) } else { false });
        // Restore the cell.
        board[i][j] = temp;
        found
    }

    let mut board_mut = board.clone();
    for i in 0..rows {
        for j in 0..cols {
            if dfs(&mut board_mut, &word_chars, i, j, 0, rows, cols) {
                return true;
            }
        }
    }
    false
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 28: Word Search");
    println!("------------------------");
    println!("Determine if a given word exists in a grid of characters using adjacent cells.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_search_found() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        assert_eq!(word_search(board, "ABCCED".to_string()), true);
    }

    #[test]
    fn test_word_search_not_found() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        assert_eq!(word_search(board, "SEE".to_string()), true);
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        assert_eq!(word_search(board, "ABCB".to_string()), false);
    }
}
