/*
Question 18: Letter Combinations of a Phone Number

Question:
---------
Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
Return the answer in any order.

Example:
---------
Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
*/

/// Returns all possible letter combinations for the given digit string.
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    
    let mapping = vec![
        "",     // 0
        "",     // 1
        "abc",  // 2
        "def",  // 3
        "ghi",  // 4
        "jkl",  // 5
        "mno",  // 6
        "pqrs", // 7
        "tuv",  // 8
        "wxyz", // 9
    ];
    
    let mut result = Vec::new();
    let mut combination = String::new();
    
    fn backtrack(
        index: usize,
        digits: &str,
        mapping: &Vec<&str>,
        combination: &mut String,
        result: &mut Vec<String>
    ) {
        if index == digits.len() {
            result.push(combination.clone());
            return;
        }
        let digit = digits.chars().nth(index).unwrap();
        let letters = mapping[digit.to_digit(10).unwrap() as usize];
        for ch in letters.chars() {
            combination.push(ch);
            backtrack(index + 1, digits, mapping, combination, result);
            combination.pop();
        }
    }
    
    backtrack(0, &digits, &mapping, &mut combination, &mut result);
    result
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 18: Letter Combinations of a Phone Number");
    println!("--------------------------------------------------");
    println!("Return all possible letter combinations for a given digit string (digits 2-9).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let mut output = letter_combinations("23".to_string());
        output.sort();
        let mut expected = vec![
            "ad".to_string(), "ae".to_string(), "af".to_string(),
            "bd".to_string(), "be".to_string(), "bf".to_string(),
            "cd".to_string(), "ce".to_string(), "cf".to_string(),
        ];
        expected.sort();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_letter_combinations_empty() {
        assert_eq!(letter_combinations("".to_string()), Vec::<String>::new());
    }
}
