/*
Question 64: Median of Two Sorted Arrays

Question:
---------
Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
The overall run time complexity should be O(m+n) if merging, though an optimal O(log(m+n)) solution exists.
For simplicity, here we merge the arrays.
Example:
---------
Input: nums1 = [1,3], nums2 = [2]
Output: 2.0
Explanation: Merged array = [1,2,3], median is 2.
*/

/// Returns the median of two sorted arrays.
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
    let (mut i, mut j) = (0, 0);
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }
    while i < nums1.len() {
        merged.push(nums1[i]);
        i += 1;
    }
    while j < nums2.len() {
        merged.push(nums2[j]);
        j += 1;
    }
    let total = merged.len();
    if total % 2 == 1 {
        merged[total / 2] as f64
    } else {
        (merged[total / 2 - 1] as f64 + merged[total / 2] as f64) / 2.0
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 64: Median of Two Sorted Arrays");
    println!("----------------------------------------");
    println!("Given two sorted arrays, find the median of the combined sorted array.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays_odd() {
        assert_eq!(find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
    }

    #[test]
    fn test_find_median_sorted_arrays_even() {
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
    }
}
