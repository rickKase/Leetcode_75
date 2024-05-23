/*
Question 8: Search Insert Position

Question:
---------
Given a sorted array of distinct integers and a target value, return the index if the target is found.
If not, return the index where it would be if it were inserted in order.
*/

/// Returns the index where the target is found or should be inserted.
pub fn search_insert(nums: Vec<i32>, target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 8: Search Insert Position");
    println!("----------------------------------");
    println!("Given a sorted array of distinct integers and a target value,");
    println!("return the index if the target is found.");
    println!("If not, return the index where it would be if it were inserted in order.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![1,3,5,6], 5), 2);
        assert_eq!(search_insert(vec![1,3,5,6], 2), 1);
        assert_eq!(search_insert(vec![1,3,5,6], 7), 4);
        assert_eq!(search_insert(vec![1,3,5,6], 0), 0);
    }
}
