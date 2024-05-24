/*
Question 16: Remove Nth Node From End of List

Question:
---------
Given a linked list, remove the nth node from the end of list and return its head.
Example:
---------
Input: 1->2->3->4->5, n = 2
Output: 1->2->3->5
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

/// Removes the nth node from the end of the list and returns the new head.
/// This solution computes the length of the list first and then advances a pointer to the node
/// just before the one to remove.
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    // Compute the length of the list.
    let mut length = 0;
    {
         let mut curr = dummy.as_ref();
         while let Some(node) = curr.next.as_ref() {
              length += 1;
              curr = node;
         }
    }
    // The node to remove is the (length - n + 1)th node (1-indexed),
    // so the node before it is at position (length - n) starting from dummy (which is at position 0).
    let steps = length - n as usize;
    let mut slow = dummy.as_mut();
    for _ in 0..steps {
         slow = slow.next.as_mut().unwrap();
    }
    // Remove the nth node.
    slow.next = slow.next.take().and_then(|node| node.next);
    dummy.next
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 16: Remove Nth Node From End of List");
    println!("---------------------------------------------");
    println!("Given a linked list, remove the nth node from the end and return the head.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: Convert a vector to a linked list.
    fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
         let mut head = None;
         for num in nums.into_iter().rev() {
              let mut node = Box::new(ListNode::new(num));
              node.next = head;
              head = Some(node);
         }
         head
    }

    /// Helper: Convert a linked list to a vector.
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
         let mut vec = Vec::new();
         while let Some(node) = head {
              vec.push(node.val);
              head = node.next;
         }
         vec
    }

    #[test]
    fn test_remove_nth_from_end() {
         let list = vec_to_list(vec![1, 2, 3, 4, 5]);
         let result = remove_nth_from_end(list, 2);
         assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_remove_nth_from_end_remove_head() {
         let list = vec_to_list(vec![1, 2, 3]);
         let result = remove_nth_from_end(list, 3);
         assert_eq!(list_to_vec(result), vec![2, 3]);
    }
}
