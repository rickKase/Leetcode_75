/*
Question 65: Minimum Window Substring

Question:
---------
Given two strings s and t, return the minimum window in s which contains all the characters in t.
If there is no such window, return an empty string "".
Example:
---------
Input: s = "ADOBECODEBANC", t = "ABC"
Output: "BANC"
*/

use std::collections::HashMap;

/// Returns the minimum window substring of s that contains all characters of t.
pub fn min_window(s: String, t: String) -> String {
    if t.is_empty() || s.len() < t.len() {
        return "".to_string();
    }
    
    let mut dict_t = HashMap::new();
    for c in t.chars() {
        *dict_t.entry(c).or_insert(0) += 1;
    }
    let required = dict_t.len();

    let s_chars: Vec<char> = s.chars().collect();
    let mut l = 0;
    let mut r = 0;
    let mut formed = 0;
    let mut window_counts = HashMap::new();
    let mut ans = (usize::MAX, 0, 0); // (window length, left, right)

    while r < s_chars.len() {
        let c = s_chars[r];
        *window_counts.entry(c).or_insert(0) += 1;

        if dict_t.contains_key(&c) && window_counts[&c] == dict_t[&c] {
            formed += 1;
        }

        while l <= r && formed == required {
            // Update answer if smaller window found.
            if r - l + 1 < ans.0 {
                ans = (r - l + 1, l, r);
            }
            let c = s_chars[l];
            *window_counts.entry(c).or_insert(0) -= 1;
            if dict_t.contains_key(&c) && window_counts[&c] < dict_t[&c] {
                formed -= 1;
            }
            l += 1;
        }
        r += 1;
    }
    if ans.0 == usize::MAX {
        "".to_string()
    } else {
        s_chars[ans.1..=ans.2].iter().collect()
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 65: Minimum Window Substring");
    println!("-------------------------------------");
    println!("Find the smallest substring in s that contains all characters of t.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window_substring() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        assert_eq!(min_window(s, t), "BANC".to_string());
    }

    #[test]
    fn test_min_window_substring_no_solution() {
        let s = "A".to_string();
        let t = "AA".to_string();
        assert_eq!(min_window(s, t), "".to_string());
    }
}
