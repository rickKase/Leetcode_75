/*
Question 59: Permutations II

Question:
---------
Given a collection of numbers that might contain duplicates, return all possible unique permutations.
The answer can be returned in any order.

Example:
---------
Input: nums = [1,1,2]
Output: [
  [1,1,2],
  [1,2,1],
  [2,1,1]
]
*/

pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    nums.sort();
    let mut used = vec![false; nums.len()];
    let mut current = Vec::new();
    backtrack(&nums, &mut used, &mut current, &mut res);
    res
}

fn backtrack(nums: &Vec<i32>, used: &mut Vec<bool>, current: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if current.len() == nums.len() {
        res.push(current.clone());
        return;
    }
    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
            continue;
        }
        used[i] = true;
        current.push(nums[i]);
        backtrack(nums, used, current, res);
        current.pop();
        used[i] = false;
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 59: Permutations II");
    println!("----------------------------");
    println!("Return all unique permutations of a list of numbers that may contain duplicates.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_permute_unique() {
        let mut res = permute_unique(vec![1,1,2]);
        // Convert result to a set for order-insensitive comparison.
        let res_set: HashSet<Vec<i32>> = res.drain(..).collect();
        let expected: HashSet<Vec<i32>> = vec![
            vec![1,1,2],
            vec![1,2,1],
            vec![2,1,1],
        ].into_iter().collect();
        assert_eq!(res_set, expected);
    }
}
