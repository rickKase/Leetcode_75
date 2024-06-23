/*
Question 75: Find Peak Element

Question:
---------
A peak element is an element that is strictly greater than its neighbors.
Given an integer array nums, find a peak element and return its index.
If the array contains multiple peaks, return the index to any one of them.
You may assume nums[-1] = nums[n] = -∞.
Example:
---------
Input: nums = [1,2,3,1]
Output: 2
Explanation: 3 is a peak element, and your function should return index 2.
*/

/// Finds and returns the index of a peak element using binary search.
/// The algorithm runs in O(log n) time.
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return -1;
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[mid + 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as i32
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 75: Find Peak Element");
    println!("------------------------------");
    println!("Given an array, find the index of a peak element (an element that is strictly greater than its neighbors).");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_peak_element() {
        assert_eq!(find_peak_element(vec![1,2,3,1]), 2);
        // In case of multiple peaks, any valid peak index is acceptable.
        let index = find_peak_element(vec![1,2,1,3,5,6,4]);
        let nums = vec![1,2,1,3,5,6,4];
        if (index as usize) > 0 && (index as usize) < (nums.len() - 1) {
            assert!(nums[index as usize] > nums[index as usize - 1]);
            assert!(nums[index as usize] > nums[index as usize + 1]);
        }
    }
}
