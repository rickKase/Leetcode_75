/*
Question 62: Rotate Array

Question:
---------
Given an array, rotate the array to the right by k steps, where k is non-negative.
You must do this in-place with O(1) extra space.
Example:
---------
Input: nums = [1,2,3,4,5,6,7], k = 3
Output: [5,6,7,1,2,3,4]
*/

/// Rotates the array to the right by k steps in-place.
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    if n == 0 {
        return;
    }
    let k = (k as usize) % n;
    if k == 0 {
        return;
    }
    // Helper function to reverse a slice in-place.
    fn reverse(slice: &mut [i32]) {
        let mut left = 0;
        let mut right = slice.len().saturating_sub(1);
        while left < right {
            slice.swap(left, right);
            left += 1;
            right = right.saturating_sub(1);
        }
    }
    reverse(&mut nums[..]);
    reverse(&mut nums[..k]);
    reverse(&mut nums[k..]);
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 62: Rotate Array");
    println!("--------------------------");
    println!("Rotate an array to the right by k steps in-place.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rotate_no_change() {
        let mut nums = vec![1, 2, 3];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![1, 2, 3]);
    }
}
