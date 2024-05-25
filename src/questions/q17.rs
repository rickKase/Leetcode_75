/*
Question 17: Valid Anagram

Question:
---------
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

Example:
---------
Input: s = "anagram", t = "nagaram"
Output: true
*/

use std::collections::HashMap;

/// Returns true if t is an anagram of s.
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut count = HashMap::new();
    for ch in s.chars() {
        *count.entry(ch).or_insert(0) += 1;
    }
    for ch in t.chars() {
        if let Some(val) = count.get_mut(&ch) {
            *val -= 1;
            if *val < 0 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 17: Valid Anagram");
    println!("-------------------------");
    println!("Determine if one string is an anagram of another.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram_true() {
        assert_eq!(is_anagram("anagram".to_string(), "nagaram".to_string()), true);
    }

    #[test]
    fn test_is_anagram_false() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
