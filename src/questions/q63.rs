/*
Question 63: Longest Substring Without Repeating Characters

Question:
---------
Given a string, find the length of the longest substring without repeating characters.
Example:
---------
Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.
*/

use std::collections::HashMap;

/// Returns the length of the longest substring without repeating characters.
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 0;
    let mut start = 0;
    let mut char_index = HashMap::new();
    let s_chars: Vec<char> = s.chars().collect();
    
    for (i, &c) in s_chars.iter().enumerate() {
        if let Some(&prev_index) = char_index.get(&c) {
            // Move start to the right of the previous index if necessary.
            start = start.max(prev_index + 1);
        }
        char_index.insert(c, i);
        max_len = max_len.max(i - start + 1);
    }
    max_len as i32
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 63: Longest Substring Without Repeating Characters");
    println!("-----------------------------------------------------------");
    println!("Given a string, determine the length of the longest substring without repeating characters.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }
}
