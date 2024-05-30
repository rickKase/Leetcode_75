/*
Question 30: Subsets

Question:
---------
Given an integer array nums, return all possible subsets (the power set).
The solution set must not contain duplicate subsets.

Example:
---------
Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
*/

/// Returns all possible subsets of the given array.
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut subset = Vec::new();
    fn backtrack(start: usize, nums: &Vec<i32>, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(subset.clone());
        for i in start..nums.len() {
            subset.push(nums[i]);
            backtrack(i + 1, nums, subset, res);
            subset.pop();
        }
    }
    backtrack(0, &nums, &mut subset, &mut res);
    res
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 30: Subsets");
    println!("--------------------");
    println!("Return all possible subsets (the power set) of a given array.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_subsets() {
        let mut res = subsets(vec![1, 2, 3]);
        // Convert each subset into a sorted tuple and collect into a set for comparison.
        let res_set: HashSet<Vec<i32>> = res.iter_mut().map(|v| { v.sort(); v.clone() }).collect();

        let expected = vec![
            vec![], vec![1], vec![2], vec![1,2],
            vec![3], vec![1,3], vec![2,3], vec![1,2,3]
        ];
        let expected_set: HashSet<Vec<i32>> = expected.into_iter().map(|mut v| { v.sort(); v }).collect();

        assert_eq!(res_set, expected_set);
    }
}
