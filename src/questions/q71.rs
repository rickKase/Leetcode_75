/*
Question 71: Merge Sorted Array

Question:
---------
You are given two integer arrays `nums1` and `nums2`, sorted in non-decreasing order, and two integers `m` and `n` representing the number of elements in `nums1` and `nums2` respectively.
`nums1` has a length of `m + n`, where the first `m` elements denote the elements that should be merged, and the last `n` elements are set to 0 and should be ignored.
Merge `nums2` into `nums1` as one sorted array **in-place**.
Example:
---------
Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
Output: [1,2,2,3,5,6]
*/

/// Merges two sorted arrays into nums1 in-place.
pub fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &Vec<i32>, n: usize) {
    // Start merging from the end.
    let mut i = m as i32 - 1;
    let mut j = n as i32 - 1;
    let mut k = (m + n) as i32 - 1;

    while i >= 0 && j >= 0 {
        if nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }

    // If any elements remain in nums2, copy them.
    while j >= 0 {
        nums1[k as usize] = nums2[j as usize];
        j -= 1;
        k -= 1;
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 71: Merge Sorted Array");
    println!("--------------------------------");
    println!("Merge two sorted arrays, where nums1 has enough space to hold nums2, into one sorted array in-place.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merge() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let nums2 = vec![2,5,6];
        merge(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1,2,2,3,5,6]);
    }
}
