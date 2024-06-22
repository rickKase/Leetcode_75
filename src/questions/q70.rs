/*
Question 70: Search in Rotated Sorted Array

Question:
---------
Given an integer array nums sorted in ascending order that is rotated at an unknown pivot, and a target value, return the index if the target is found, otherwise return -1.
You may assume no duplicate exists in the array.
Example:
---------
Input: nums = [4,5,6,7,0,1,2], target = 0
Output: 4
*/

/// Searches for target in a rotated sorted array and returns its index or -1 if not found.
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }
    let (mut left, mut right) = (0, nums.len() as i32 - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid;
        }
        // Determine which half is properly sorted.
        if nums[left as usize] <= nums[mid as usize] {
            // Left half is sorted.
            if nums[left as usize] <= target && target < nums[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // Right half is sorted.
            if nums[mid as usize] < target && target <= nums[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 70: Search in Rotated Sorted Array");
    println!("------------------------------------------");
    println!("Given a rotated sorted array with no duplicates, find the index of a target value, or return -1 if not found.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_found() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 6), 2);
    }
    
    #[test]
    fn test_search_not_found() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
    }
}
