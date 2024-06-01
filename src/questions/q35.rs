/*
Question 35: Word Break

Question:
---------
Given a string `s` and a dictionary of strings `wordDict`, determine if `s` can be segmented into a space-separated sequence of one or more dictionary words.
Note: The same word in the dictionary may be reused multiple times in the segmentation.

Example:
---------
Input: s = "leetcode", wordDict = ["leet","code"]
Output: true
Explanation: "leetcode" can be segmented as "leet code".
*/

use std::collections::HashSet;

/// Returns true if the string can be segmented into one or more words from the dictionary.
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let word_set: HashSet<String> = word_dict.into_iter().collect();
    let n = s.len();
    let s_chars: Vec<char> = s.chars().collect();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    
    for i in 1..=n {
        for j in 0..i {
            if dp[j] {
                let substring: String = s_chars[j..i].iter().collect();
                if word_set.contains(&substring) {
                    dp[i] = true;
                    break;
                }
            }
        }
    }
    dp[n]
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 35: Word Break");
    println!("-----------------------");
    println!("Determine if a string can be segmented into a sequence of dictionary words.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_word_break_true() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        assert_eq!(word_break(s, word_dict), true);
    }
    
    #[test]
    fn test_word_break_false() {
        let s = "catsandog".to_string();
        let word_dict = vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string()
        ];
        assert_eq!(word_break(s, word_dict), false);
    }
}
