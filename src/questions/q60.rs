/*
Question 60: Longest Valid Parentheses

Question:
---------
Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

Example:
---------
Input: s = "(()"
Output: 2

Input: s = ")()())"
Output: 4
*/

pub fn longest_valid_parentheses(s: String) -> i32 {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    let s_chars: Vec<char> = s.chars().collect();
    let mut dp = vec![0; n];
    let mut max_len = 0;
    for i in 1..n {
        if s_chars[i] == ')' {
            if s_chars[i - 1] == '(' {
                dp[i] = if i >= 2 { dp[i - 2] } else { 0 } + 2;
            } else if i - dp[i - 1] > 0 && s_chars[i - dp[i - 1] - 1] == '(' {
                dp[i] = dp[i - 1] + 2;
                if i - dp[i - 1] >= 2 {
                    dp[i] += dp[i - dp[i - 1] - 2];
                }
            }
            max_len = max_len.max(dp[i]);
        }
    }
    max_len as i32
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 60: Longest Valid Parentheses");
    println!("--------------------------------------");
    println!("Find the length of the longest valid (well-formed) parentheses substring in the given string.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses_case1() {
        assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn test_longest_valid_parentheses_case2() {
        assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn test_longest_valid_parentheses_case3() {
        assert_eq!(longest_valid_parentheses("".to_string()), 0);
    }
}
