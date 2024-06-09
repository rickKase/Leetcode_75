/*
Question 53: Spiral Matrix

Question:
---------
Given an m x n matrix, return all elements of the matrix in spiral order.

Example:
---------
Input: matrix = [
  [ 1, 2, 3 ],
  [ 4, 5, 6 ],
  [ 7, 8, 9 ]
]
Output: [1,2,3,6,9,8,7,4,5]
*/

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut top = 0;
    let mut bottom = matrix.len() as i32 - 1;
    let mut left = 0;
    let mut right = matrix[0].len() as i32 - 1;

    while top <= bottom && left <= right {
        // Traverse from left to right.
        for j in left..=right {
            result.push(matrix[top as usize][j as usize]);
        }
        top += 1;

        // Traverse from top to bottom.
        for i in top..=bottom {
            result.push(matrix[i as usize][right as usize]);
        }
        right -= 1;

        if top <= bottom {
            // Traverse from right to left.
            for j in (left..=right).rev() {
                result.push(matrix[bottom as usize][j as usize]);
            }
            bottom -= 1;
        }

        if left <= right {
            // Traverse from bottom to top.
            for i in (top..=bottom).rev() {
                result.push(matrix[i as usize][left as usize]);
            }
            left += 1;
        }
    }
    result
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 53: Spiral Matrix");
    println!("--------------------------");
    println!("Return all elements of a matrix in spiral order.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(spiral_order(matrix), vec![1,2,3,6,9,8,7,4,5]);
    }
}
