/*
Question 54: Spiral Matrix II

Question:
---------
Given a positive integer n, generate an n x n matrix filled with elements from 1 to n² in spiral order.

Example:
---------
Input: n = 3
Output: [
 [ 1, 2, 3 ],
 [ 8, 9, 4 ],
 [ 7, 6, 5 ]
]
*/

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut matrix = vec![vec![0; n]; n];
    let mut num = 1;
    let mut top = 0;
    let mut bottom = n as i32 - 1;
    let mut left = 0;
    let mut right = n as i32 - 1;

    while top <= bottom && left <= right {
        // Fill top row.
        for j in left..=right {
            matrix[top as usize][j as usize] = num;
            num += 1;
        }
        top += 1;

        // Fill right column.
        for i in top..=bottom {
            matrix[i as usize][right as usize] = num;
            num += 1;
        }
        right -= 1;

        if top <= bottom {
            // Fill bottom row.
            for j in (left..=right).rev() {
                matrix[bottom as usize][j as usize] = num;
                num += 1;
            }
            bottom -= 1;
        }

        if left <= right {
            // Fill left column.
            for i in (top..=bottom).rev() {
                matrix[i as usize][left as usize] = num;
                num += 1;
            }
            left += 1;
        }
    }
    matrix
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 54: Spiral Matrix II");
    println!("-----------------------------");
    println!("Generate an n x n matrix filled with numbers 1 to n² in spiral order.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_matrix() {
        let expected = vec![
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5],
        ];
        assert_eq!(generate_matrix(3), expected);
    }
}
