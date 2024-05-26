/*
Question 23: Decode Ways

Question:
---------
A message containing letters from A-Z is being encoded to numbers using the following mapping:
'A' -> "1", 'B' -> "2", ..., 'Z' -> "26". Given a non-empty string containing only digits, determine the total number of ways to decode it.

Example:
---------
Input: s = "226"
Output: 3
Explanation: "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
*/

/// Returns the number of ways to decode the given string.
pub fn num_decodings(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let n = s.len();
    let s_bytes = s.as_bytes();
    let mut dp = vec![0; n + 1];
    dp[n] = 1; // Base case: empty string has 1 way to decode.
    
    for i in (0..n).rev() {
        if s_bytes[i] == b'0' {
            dp[i] = 0;
        } else {
            dp[i] = dp[i + 1];
            if i + 1 < n {
                let two_digit = (s_bytes[i] - b'0') * 10 + (s_bytes[i + 1] - b'0');
                if two_digit >= 10 && two_digit <= 26 {
                    dp[i] += dp[i + 2];
                }
            }
        }
    }
    dp[0]
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 23: Decode Ways");
    println!("------------------------");
    println!("Count the number of ways to decode a numeric string into letters (A=1, B=2, ..., Z=26).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_decodings() {
        assert_eq!(num_decodings("12".to_string()), 2); // "AB" or "L"
        assert_eq!(num_decodings("226".to_string()), 3);
        assert_eq!(num_decodings("0".to_string()), 0);
        assert_eq!(num_decodings("10".to_string()), 1);
    }
}
