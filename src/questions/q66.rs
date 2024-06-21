/*
Question 66: Plus One

Question:
---------
Given a non-empty array of digits representing a non-negative integer, plus one to the integer.
The digits are stored such that the most significant digit is at the head of the list.
Return the resulting array of digits.
Example:
---------
Input: digits = [1,2,3]
Output: [1,2,4]
*/

/// Adds one to the number represented by the vector of digits.
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    for digit in digits.iter_mut().rev() {
        let sum = *digit + carry;
        *digit = sum % 10;
        carry = sum / 10;
        if carry == 0 {
            break;
        }
    }
    if carry != 0 {
        digits.insert(0, carry);
    }
    digits
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 66: Plus One");
    println!("---------------------");
    println!("Given an array of digits representing a non-negative integer, add one to the integer and return the resulting array of digits.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plus_one_no_carry() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }
    
    #[test]
    fn test_plus_one_with_carry() {
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
