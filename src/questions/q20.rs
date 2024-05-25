/*
Question 20: Merge Intervals

Question:
---------
Given an array of intervals where intervals[i] = [start_i, end_i], merge all overlapping intervals,
and return an array of the non-overlapping intervals that cover all the intervals in the input.

Example:
---------
Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
*/

/// Merges overlapping intervals and returns the merged intervals.
pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }
    // Sort intervals by the starting value.
    intervals.sort_by_key(|interval| interval[0]);
    let mut merged = Vec::new();
    let mut current = intervals[0].clone();
    for interval in intervals.into_iter().skip(1) {
        if interval[0] <= current[1] {
            current[1] = current[1].max(interval[1]);
        } else {
            merged.push(current);
            current = interval;
        }
    }
    merged.push(current);
    merged
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 20: Merge Intervals");
    println!("----------------------------");
    println!("Merge all overlapping intervals from a list of intervals.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18],
        ];
        let result = merge_intervals(intervals);
        let expected = vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_intervals_no_overlap() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        let result = merge_intervals(intervals);
        let expected = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        assert_eq!(result, expected);
    }
}
