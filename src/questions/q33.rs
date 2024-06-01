/*
Question 33: Merge k Sorted Lists

Question:
---------
You are given an array of k linked-lists `lists`, each linked-list is sorted in ascending order.
Merge all the linked-lists into one sorted linked-list and return it.
Example:
---------
Input: lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]
*/

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
         ListNode { val, next: None }
    }
}

// Implement natural order (ascending) for ListNode.
impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
         self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
         Some(self.cmp(other))
    }
}

/// Merges k sorted linked lists into one sorted linked list.
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();
    for list in lists.into_iter() {
         if let Some(node) = list {
              heap.push(Reverse(node));
         }
    }
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    while let Some(Reverse(mut node)) = heap.pop() {
         if let Some(next) = node.next.take() {
              heap.push(Reverse(next));
         }
         tail.next = Some(node);
         tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 33: Merge k Sorted Lists");
    println!("---------------------------------");
    println!("Merge k sorted linked lists into one sorted linked list.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: Convert a vector to a linked list.
    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
         let mut current = None;
         for &num in vec.iter().rev() {
              let mut node = Box::new(ListNode::new(num));
              node.next = current;
              current = Some(node);
         }
         current
    }

    // Helper: Convert a linked list to a vector.
    fn list_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
         let mut vec = Vec::new();
         while let Some(node) = list {
              vec.push(node.val);
              list = node.next;
         }
         vec
    }

    #[test]
    fn test_merge_k_lists() {
         let list1 = vec_to_list(vec![1,4,5]);
         let list2 = vec_to_list(vec![1,3,4]);
         let list3 = vec_to_list(vec![2,6]);
         let merged = merge_k_lists(vec![list1, list2, list3]);
         assert_eq!(list_to_vec(merged), vec![1,1,2,3,4,4,5,6]);
    }
}
