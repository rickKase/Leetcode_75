/*
Question 45: Sliding Window Maximum

Question:
---------
Given an array of integers and a sliding window size k, return the maximum value in each sliding window.
The window moves from left to right by one position each time.

Example:
---------
Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
Output: [3,3,5,5,6,7]
*/

use std::collections::VecDeque;

/// Returns a vector of the maximums of each sliding window of size k.
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    if n * k == 0 {
        return vec![];
    }
    if k == 1 {
        return nums;
    }
    let mut deq: VecDeque<usize> = VecDeque::new();
    let mut result = vec![];

    for i in 0..n {
        // Remove indices that are out of the current window.
        if let Some(&front) = deq.front() {
            if front <= i.saturating_sub(k) {
                deq.pop_front();
            }
        }
        // Remove indices whose corresponding values are less than nums[i].
        while let Some(&back) = deq.back() {
            if nums[back] < nums[i] {
                deq.pop_back();
            } else {
                break;
            }
        }
        deq.push_back(i);
        if i >= k - 1 {
            result.push(nums[*deq.front().unwrap()]);
        }
    }
    result
}

pub fn print_question() {
    println!("Question 45: Sliding Window Maximum");
    println!("-----------------------------------");
    println!("Given an array and a window size k, find the maximum value in each sliding window.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sliding_window() {
        assert_eq!(max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }
}
