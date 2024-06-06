/*
Question 41: Trapping Rain Water

Question:
---------
Given n non-negative integers representing an elevation map where the width of each bar is 1,
compute how much water it can trap after raining.

Example:
---------
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
*/

/// Returns the total amount of trapped rain water.
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    }
    let mut left = 0;
    let mut right = n - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut water = 0;
    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                water += right_max - height[right];
            }
            right -= 1;
        }
    }
    water
}

pub fn print_question() {
    println!("Question 41: Trapping Rain Water");
    println!("-------------------------------");
    println!("Given an elevation map, compute the amount of trapped rain water.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trap() {
        assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(trap(vec![4,2,0,3,2,5]), 9);
    }
}
