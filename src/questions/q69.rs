/*
Question 69: Add Two Numbers

Question:
---------
You are given two non-empty linked lists representing two non-negative integers.
The digits are stored in reverse order, and each node contains a single digit.
Add the two numbers and return the sum as a linked list.
Example:
---------
Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.
*/

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
         ListNode { val, next: None }
    }
}

/// Adds two numbers represented by linked lists and returns the sum as a linked list.
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut p = l1;
    let mut q = l2;
    let mut current = &mut dummy;
    let mut carry = 0;

    while p.is_some() || q.is_some() || carry != 0 {
        let sum = carry +
            p.as_ref().map_or(0, |node| node.val) +
            q.as_ref().map_or(0, |node| node.val);
        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
        if let Some(node) = p { p = node.next; }
        if let Some(node) = q { q = node.next; }
    }
    dummy.next
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 69: Add Two Numbers");
    println!("----------------------------");
    println!("Add two numbers represented by linked lists (digits stored in reverse order) and return the sum as a linked list.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: Converts a vector of integers to a linked list.
    fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
         let mut head = None;
         for num in nums.into_iter().rev() {
             let mut node = Box::new(ListNode::new(num));
             node.next = head;
             head = Some(node);
         }
         head
    }

    /// Helper: Converts a linked list to a vector of integers.
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
         let mut vec = Vec::new();
         while let Some(node) = head {
             vec.push(node.val);
             head = node.next;
         }
         vec
    }

    #[test]
    fn test_add_two_numbers() {
         let l1 = vec_to_list(vec![2,4,3]);
         let l2 = vec_to_list(vec![5,6,4]);
         let sum = add_two_numbers(l1, l2);
         assert_eq!(list_to_vec(sum), vec![7,0,8]);
    }
}
