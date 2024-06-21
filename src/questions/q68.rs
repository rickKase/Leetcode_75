/*
Question 68: Length of Last Word

Question:
---------
Given a string s consisting of words and spaces, return the length of the last word in s.
A word is a maximal substring consisting of non-space characters only.
Example:
---------
Input: s = "Hello World"
Output: 5
*/

/// Returns the length of the last word in the given string.
pub fn length_of_last_word(s: String) -> i32 {
    s.trim()
     .split_whitespace()
     .last()
     .map(|word| word.len() as i32)
     .unwrap_or(0)
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 68: Length of Last Word");
    println!("--------------------------------");
    println!("Given a string, return the length of the last word (a maximal substring of non-space characters).");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
        assert_eq!(length_of_last_word("".to_string()), 0);
    }
}
