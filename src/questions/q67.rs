/*
Question 67: Add Binary

Question:
---------
Given two binary strings a and b, return their sum as a binary string.
Example:
---------
Input: a = "11", b = "1"
Output: "100"
*/

/// Adds two binary strings and returns their sum as a binary string.
pub fn add_binary(a: String, b: String) -> String {
    let (mut i, mut j) = (a.len() as i32 - 1, b.len() as i32 - 1);
    let mut carry = 0;
    let mut result = String::new();
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    while i >= 0 || j >= 0 || carry != 0 {
        let mut sum = carry;
        if i >= 0 {
            sum += a_chars[i as usize].to_digit(10).unwrap() as i32;
            i -= 1;
        }
        if j >= 0 {
            sum += b_chars[j as usize].to_digit(10).unwrap() as i32;
            j -= 1;
        }
        carry = sum / 2;
        result.push(std::char::from_digit((sum % 2) as u32, 10).unwrap());
    }
    result.chars().rev().collect()
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 67: Add Binary");
    println!("-----------------------");
    println!("Given two binary strings, return their sum as a binary string.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_binary_simple() {
        assert_eq!(add_binary("11".to_string(), "1".to_string()), "100".to_string());
    }
    
    #[test]
    fn test_add_binary_longer() {
        assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101".to_string());
    }
}
