/*
Question 46: Number of Islands

Question:
---------
Given a 2D grid map of '1's (land) and '0's (water), count the number of islands.
An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
You may assume all four edges of the grid are all surrounded by water.

Example:
---------
Input: grid = [
  ['1','1','0','0','0'],
  ['1','1','0','0','0'],
  ['0','0','1','0','0'],
  ['0','0','0','1','1']
]
Output: 3
*/

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let mut grid = grid; // make mutable copy
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' {
                count += 1;
                dfs(&mut grid, i, j, rows, cols);
            }
        }
    }
    count
}

fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, rows: usize, cols: usize) {
    if i >= rows || j >= cols || grid[i][j] != '1' {
        return;
    }
    // Mark as visited.
    grid[i][j] = '0';
    // Move in four directions.
    if i > 0 {
        dfs(grid, i - 1, j, rows, cols);
    }
    if i + 1 < rows {
        dfs(grid, i + 1, j, rows, cols);
    }
    if j > 0 {
        dfs(grid, i, j - 1, rows, cols);
    }
    if j + 1 < cols {
        dfs(grid, i, j + 1, rows, cols);
    }
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 46: Number of Islands");
    println!("------------------------------");
    println!("Count the number of islands in a 2D grid of '1's (land) and '0's (water).");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_num_islands() {
        let grid = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1'],
        ];
        assert_eq!(num_islands(grid), 3);
    }
}
