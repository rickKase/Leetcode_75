/*
Question 27: Valid Sudoku

Question:
---------
Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be validated 
according to the following rules:
  - Each row must contain the digits 1-9 without repetition.
  - Each column must contain the digits 1-9 without repetition.
  - Each of the 9 3x3 sub-boxes of the grid must contain the digits 1-9 without repetition.

The Sudoku board is represented as a vector of vector of characters, where '.' indicates an empty cell.

Example:
---------
Input: 
[
  ['5','3','.','.','7','.','.','.','.'],
  ['6','.','.','1','9','5','.','.','.'],
  ['.','9','8','.','.','.','.','6','.'],
  ['8','.','.','.','6','.','.','.','3'],
  ['4','.','.','8','.','3','.','.','1'],
  ['7','.','.','.','2','.','.','.','6'],
  ['.','6','.','.','.','.','2','8','.'],
  ['.','.','.','4','1','9','.','.','5'],
  ['.','.','.','.','8','.','.','7','9']
]
Output: true
*/

/// Returns true if the given 9x9 board is a valid Sudoku.
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let n = 9;
    let mut rows = vec![vec![false; 9]; n];
    let mut cols = vec![vec![false; 9]; n];
    let mut boxes = vec![vec![false; 9]; n];

    for i in 0..n {
        for j in 0..n {
            let c = board[i][j];
            if c != '.' {
                let num = (c as u8 - b'1') as usize;
                let box_index = (i / 3) * 3 + (j / 3);
                if rows[i][num] || cols[j][num] || boxes[box_index][num] {
                    return false;
                }
                rows[i][num] = true;
                cols[j][num] = true;
                boxes[box_index][num] = true;
            }
        }
    }
    true
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 27: Valid Sudoku");
    println!("-------------------------");
    println!("Determine if a 9x9 Sudoku board is valid (only filled cells are validated).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sudoku_true() {
        let board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert_eq!(is_valid_sudoku(board), true);
    }

    #[test]
    fn test_valid_sudoku_false() {
        let board = vec![
            vec!['8','3','.','.','7','.','.','.','.'], // Duplicate '8' in first row and first box.
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert_eq!(is_valid_sudoku(board), false);
    }
}
