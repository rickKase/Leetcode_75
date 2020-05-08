/*
Question 7: Valid Parentheses

Question:
---------
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
determine if the input string is valid.

An input string is valid if:
- Open brackets must be closed by the same type of brackets.
- Open brackets must be closed in the correct order.
*/

/// Checks if the given string of parentheses is valid.
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            },
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            },
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            },
            _ => {}
        }
    }
    stack.is_empty()
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 7: Valid Parentheses");
    println!("-----------------------------");
    println!("Given a string s containing just the characters '(', ')', '{{', '}}', '[' and ']',");
    println!("determine if the input string is valid.");
    println!("An input string is valid if:");
    println!(" - Open brackets must be closed by the same type of brackets.");
    println!(" - Open brackets must be closed in the correct order.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
        assert_eq!(is_valid("([)]".to_string()), false);
        assert_eq!(is_valid("{[]}".to_string()), true);
    }
}
