/*
Question 74: Sort Colors

Question:
---------
Given an array `nums` with n objects colored red, white, or blue (represented by 0, 1, and 2), sort them in-place so that objects of the same color are adjacent,
with the colors in the order red, white, and blue.
Example:
---------
Input: nums = [2,0,2,1,1,0]
Output: [0,0,1,1,2,2]
*/

/// Sorts the colors in-place using the Dutch National Flag algorithm.
pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut low, mut mid) = (0, 0);
    let mut high = nums.len().saturating_sub(1);
    while mid <= high {
        match nums[mid] {
            0 => {
                nums.swap(low, mid);
                low += 1;
                mid += 1;
            },
            1 => {
                mid += 1;
            },
            2 => {
                nums.swap(mid, high);
                if high == 0 { break; }
                high -= 1;
            },
            _ => { mid += 1; }
        }
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 74: Sort Colors");
    println!("------------------------");
    println!("Given an array with values 0, 1, and 2, sort the array in-place so that all 0s come first, then 1s, then 2s.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2,0,2,1,1,0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0,0,1,1,2,2]);
    }
}
