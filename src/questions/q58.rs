/*
Question 58: Generate Parentheses

Question:
---------
Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

Example:
---------
Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]
*/

pub fn generate_parentheses(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut current = String::new();
    backtrack(n, n, &mut current, &mut res);
    res
}

fn backtrack(open: i32, close: i32, current: &mut String, res: &mut Vec<String>) {
    if open == 0 && close == 0 {
        res.push(current.clone());
        return;
    }
    if open > 0 {
        current.push('(');
        backtrack(open - 1, close, current, res);
        current.pop();
    }
    if close > open {
        current.push(')');
        backtrack(open, close - 1, current, res);
        current.pop();
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 58: Generate Parentheses");
    println!("---------------------------------");
    println!("Generate all combinations of well-formed parentheses for n pairs.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parentheses() {
        let mut res = generate_parentheses(3);
        res.sort();
        let mut expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        expected.sort();
        assert_eq!(res, expected);
    }
}
