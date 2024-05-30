/*
Question 29: Combination Sum

Question:
---------
Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target.
The same number may be chosen from candidates an unlimited number of times.
Two combinations are unique if the frequency of at least one of the chosen numbers is different.

Example:
---------
Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
*/

/// Returns all unique combinations of candidates where the chosen numbers sum to target.
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut comb = Vec::new();
    fn backtrack(start: usize, candidates: &Vec<i32>, target: i32, comb: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if target == 0 {
            res.push(comb.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > target {
                continue;
            }
            comb.push(candidates[i]);
            backtrack(i, candidates, target - candidates[i], comb, res);
            comb.pop();
        }
    }
    let mut sorted_candidates = candidates;
    sorted_candidates.sort();
    backtrack(0, &sorted_candidates, target, &mut comb, &mut res);
    res
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 29: Combination Sum");
    println!("----------------------------");
    println!("Find all unique combinations of numbers that sum to a target, where each candidate can be used unlimited times.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        let mut res = combination_sum(vec![2,3,6,7], 7);
        // Sort inner vectors and then the outer vector for comparison.
        for r in res.iter_mut() {
            r.sort();
        }
        res.sort();
        let mut expected = vec![vec![2,2,3], vec![7]];
        for r in expected.iter_mut() {
            r.sort();
        }
        expected.sort();
        assert_eq!(res, expected);
    }
}
