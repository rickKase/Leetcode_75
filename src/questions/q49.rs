/*
Question 49: Top K Frequent Elements

Question:
---------
Given a non-empty array of integers, return the k most frequent elements.
You may return the answer in any order.

Example:
---------
Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]
*/

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map = HashMap::new();
    for num in nums {
        *freq_map.entry(num).or_insert(0) += 1;
    }
    
    // Use a min-heap to keep track of the top k elements.
    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    for (&num, &freq) in &freq_map {
        heap.push(Reverse((freq, num)));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    
    heap.into_iter().map(|Reverse((_freq, num))| num).collect()
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 49: Top K Frequent Elements");
    println!("------------------------------------");
    println!("Return the k most frequent elements from an array.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_top_k_frequent() {
        let result = top_k_frequent(vec![1,1,1,2,2,3], 2);
        let mut sorted = result;
        sorted.sort();
        assert_eq!(sorted, vec![1,2]);
    }
}
