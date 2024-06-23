/*
Question 72: Reverse Linked List

Question:
---------
Given the head of a singly linked list, reverse the list, and return the reversed list.
Example:
---------
Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]
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

/// Reverses a singly linked list.
pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head.take() {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 72: Reverse Linked List");
    println!("--------------------------------");
    println!("Given the head of a singly linked list, reverse the list and return the new head.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: Converts a vector to a linked list.
    fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
         let mut head = None;
         for num in nums.into_iter().rev() {
             let mut node = Box::new(ListNode::new(num));
             node.next = head;
             head = Some(node);
         }
         head
    }
    
    /// Helper: Converts a linked list to a vector.
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
         let mut vec = Vec::new();
         while let Some(node) = head {
              vec.push(node.val);
              head = node.next;
         }
         vec
    }

    #[test]
    fn test_reverse_list() {
         let list = vec_to_list(vec![1,2,3,4,5]);
         let reversed = reverse_list(list);
         assert_eq!(list_to_vec(reversed), vec![5,4,3,2,1]);
    }
}
