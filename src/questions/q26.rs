/*
Question 26: Longest Palindromic Substring

Question:
---------
Given a string s, return the longest palindromic substring in s.

Example:
---------
Input: s = "babad"
Output: "bab" (or "aba")

Note:
A palindrome is a string that reads the same forwards and backwards.
*/

/// Returns the longest palindromic substring in the given string.
pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }
    let s_chars: Vec<char> = s.chars().collect();
    let n = s_chars.len();
    let mut start = 0;
    let mut max_len = 1;

    for i in 0..n {
        // Check for odd-length palindrome.
        let len1 = expand_around_center(&s_chars, i, i);
        // Check for even-length palindrome.
        let len2 = if i + 1 < n { expand_around_center(&s_chars, i, i + 1) } else { 0 };
        let len = len1.max(len2);
        if len > max_len {
            max_len = len;
            // Compute the new starting index.
            start = i.saturating_sub((len - 1) / 2);
        }
    }
    s_chars[start..start + max_len].iter().collect()
}

/// Helper: Expands around the center indices (left, right) and returns the length of the palindrome.
fn expand_around_center(s_chars: &Vec<char>, left: usize, right: usize) -> usize {
    let n = s_chars.len();
    let (mut l, mut r) = (left as i32, right as i32);
    while l >= 0 && (r as usize) < n && s_chars[l as usize] == s_chars[r as usize] {
        l -= 1;
        r += 1;
    }
    (r - l - 1) as usize
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 26: Longest Palindromic Substring");
    println!("-----------------------------------------");
    println!("Given a string s, return the longest palindromic substring in s.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let s1 = "babad".to_string();
        let res1 = longest_palindrome(s1);
        // Accept either "bab" or "aba" as correct.
        assert!(res1 == "bab" || res1 == "aba");

        let s2 = "cbbd".to_string();
        let res2 = longest_palindrome(s2);
        assert_eq!(res2, "bb".to_string());

        let s3 = "".to_string();
        assert_eq!(longest_palindrome(s3), "".to_string());
    }
}
