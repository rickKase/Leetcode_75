/*
Question 61: Rotate List

Question:
---------
Given the head of a linked list, rotate the list to the right by k places.
Example:
---------
Input: head = [1,2,3,4,5], k = 2
Output: [4,5,1,2,3]
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

/// Rotates the linked list to the right by k places.
/// This solution avoids cloning by splitting the list and reattaching its parts.
pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // If list is empty, has one node, or k is 0, return as-is.
    if head.is_none() || head.as_ref().unwrap().next.is_none() || k == 0 {
         return head;
    }

    // Compute the length of the list.
    let mut length = 0;
    {
         let mut curr = head.as_ref();
         while let Some(node) = curr {
              length += 1;
              curr = node.next.as_ref();
         }
    }

    // Compute the effective rotations.
    let r = (k as usize) % length;
    if r == 0 {
         return head;
    }

    // Find the node just before the new head.
    // new tail index = length - r - 1 (0-indexed)
    let split_index = length - r - 1;
    let mut curr = head.as_mut().unwrap();
    for _ in 0..split_index {
         curr = curr.next.as_mut().unwrap();
    }
    // Split the list.
    let mut new_head = curr.next.take();
    // Drop the mutable borrow so that we can later use head.
    drop(curr);

    // Traverse the new head list to its tail.
    let mut tail = new_head.as_mut().unwrap();
    while tail.next.is_some() {
         tail = tail.next.as_mut().unwrap();
    }
    // Attach the original first part (which is still in `head`) after the tail.
    tail.next = head;

    new_head
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 61: Rotate List");
    println!("------------------------");
    println!("Given the head of a linked list, rotate the list to the right by k places.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: Converts a vector of integers into a linked list.
    fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
         let mut head = None;
         for num in nums.into_iter().rev() {
             let mut node = Box::new(ListNode::new(num));
             node.next = head;
             head = Some(node);
         }
         head
    }

    /// Helper: Converts a linked list back into a vector of integers.
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
         let mut vec = Vec::new();
         while let Some(node) = head {
             vec.push(node.val);
             head = node.next;
         }
         vec
    }

    #[test]
    fn test_rotate_right() {
         let list = vec_to_list(vec![1,2,3,4,5]);
         let rotated = rotate_right(list, 2);
         assert_eq!(list_to_vec(rotated), vec![4,5,1,2,3]);
    }

    #[test]
    fn test_rotate_right_no_rotation() {
         let list = vec_to_list(vec![1,2,3]);
         // Rotating by a multiple of the length yields the same list.
         let rotated = rotate_right(list, 3);
         assert_eq!(list_to_vec(rotated), vec![1,2,3]);
    }
}
