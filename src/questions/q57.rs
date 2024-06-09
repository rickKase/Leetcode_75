/*
Question 57: Restore IP Addresses

Question:
---------
Given a string s containing only digits, return all possible valid IP addresses that can be obtained by inserting dots into s.
A valid IP address consists of exactly four integers (each integer is between 0 and 255) separated by single dots, and cannot have leading zeros.
Return the answer in any order.

Example:
---------
Input: s = "25525511135"
Output: ["255.255.11.135","255.255.111.35"]
*/

pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut res = Vec::new();
    let mut segments = Vec::new();
    backtrack(&s, 0, &mut segments, &mut res);
    res
}

fn backtrack(s: &str, start: usize, segments: &mut Vec<String>, res: &mut Vec<String>) {
    if segments.len() == 4 && start == s.len() {
        res.push(segments.join("."));
        return;
    }
    if segments.len() == 4 || start == s.len() {
        return;
    }
    for len in 1..=3 {
        if start + len > s.len() {
            break;
        }
        let segment = &s[start..start + len];
        // Skip segments with leading zeros.
        if segment.len() > 1 && segment.starts_with('0') {
            continue;
        }
        if let Ok(val) = segment.parse::<i32>() {
            if val <= 255 {
                segments.push(segment.to_string());
                backtrack(s, start + len, segments, res);
                segments.pop();
            }
        }
    }
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 57: Restore IP Addresses");
    println!("---------------------------------");
    println!("Return all valid IP addresses that can be formed by inserting dots into a string of digits.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_ip_addresses() {
        let mut res = restore_ip_addresses("25525511135".to_string());
        res.sort();
        let mut expected = vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()];
        expected.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_restore_ip_addresses_empty() {
        let res = restore_ip_addresses("0000".to_string());
        assert_eq!(res, vec!["0.0.0.0".to_string()]);
    }
}
