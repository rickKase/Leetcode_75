/*
Question 55: Permutations

Question:
---------
Given an array of distinct integers, return all possible permutations.

Example:
---------
Input: nums = [1,2,3]
Output: [
 [1,2,3],
 [1,3,2],
 [2,1,3],
 [2,3,1],
 [3,1,2],
 [3,2,1]
]
*/

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut nums = nums;
    backtrack(&mut nums, 0, &mut res);
    res
}

fn backtrack(nums: &mut Vec<i32>, start: usize, res: &mut Vec<Vec<i32>>) {
    if start >= nums.len() {
        res.push(nums.clone());
        return;
    }
    for i in start..nums.len() {
        nums.swap(start, i);
        backtrack(nums, start + 1, res);
        nums.swap(start, i); // backtrack
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 55: Permutations");
    println!("--------------------------");
    println!("Return all possible permutations of an array of distinct integers.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_permute() {
        let mut res = permute(vec![1, 2, 3]);
        // For comparison, sort each permutation and the outer vector (or compare as sets)
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        for mut v in res.drain(..) {
            // Sorting is not required if all permutations are distinct;
            // here, we insert as-is since order in each permutation matters.
            set.insert(v);
        }
        let expected: HashSet<Vec<i32>> = vec![
            vec![1,2,3],
            vec![1,3,2],
            vec![2,1,3],
            vec![2,3,1],
            vec![3,1,2],
            vec![3,2,1],
        ].into_iter().collect();
        assert_eq!(set, expected);
    }
}
