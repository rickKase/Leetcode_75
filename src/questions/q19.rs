/*
Question 19: Longest Common Prefix

Question:
---------
Write a function to find the longest common prefix string among an array of strings.
If there is no common prefix, return an empty string "".

Example:
---------
Input: ["flower", "flow", "flight"]
Output: "fl"
*/

/// Returns the longest common prefix among the provided strings.
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return "".to_string();
            }
        }
    }
    prefix
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 19: Longest Common Prefix");
    println!("----------------------------------");
    println!("Find the longest common prefix among an array of strings.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        assert_eq!(longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn test_longest_common_prefix_none() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(longest_common_prefix(strs), "".to_string());
    }
}
