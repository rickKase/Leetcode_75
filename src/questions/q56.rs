/*
Question 56: Combination Sum III

Question:
---------
Find all possible combinations of k numbers that add up to a number n, given that only numbers from 1 to 9 can be used and each combination should be a unique set of numbers.
Return a list of all possible combinations. The list can be returned in any order.

Example:
---------
Input: k = 3, n = 7
Output: [[1,2,4]]

Input: k = 3, n = 9
Output: [[1,2,6],[1,3,5],[2,3,4]]
*/

pub fn combination_sum_iii(k: i32, n: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut res = Vec::new();
    let mut comb = Vec::new();
    backtrack(1, k, n, &mut comb, &mut res);
    res
}

fn backtrack(start: i32, k: usize, n: i32, comb: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if comb.len() == k {
        if n == 0 {
            res.push(comb.clone());
        }
        return;
    }
    for num in start..=9 {
        if num > n {
            break;
        }
        comb.push(num);
        backtrack(num + 1, k, n - num, comb, res);
        comb.pop();
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 56: Combination Sum III");
    println!("--------------------------------");
    println!("Find all combinations of k numbers (from 1 to 9) that add up to n.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum_iii_case1() {
        let mut res = combination_sum_iii(3, 7);
        res.sort();
        assert_eq!(res, vec![vec![1,2,4]]);
    }

    #[test]
    fn test_combination_sum_iii_case2() {
        let mut res = combination_sum_iii(3, 9);
        res.sort();
        let mut expected = vec![vec![1,2,6], vec![1,3,5], vec![2,3,4]];
        expected.sort();
        assert_eq!(res, expected);
    }
}
