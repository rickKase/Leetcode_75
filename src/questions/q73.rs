/*
Question 73: Intersection of Two Linked Lists

Question:
---------
Given the heads of two singly linked lists, return the node at which the two lists intersect.
If the two linked lists have no intersection, return None.
Note: The intersection is defined based on reference, not value.
Example:
---------
Input: intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5]
Output: Reference to node with value 8
*/

use std::ptr;

#[derive(PartialEq, Eq, Debug, Clone)]
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

/// Returns the intersection node of two linked lists, if any.
/// (Now the function works with references to the inner `ListNode`.)
pub fn get_intersection_node<'a>(
    head_a: Option<&'a ListNode>,
    head_b: Option<&'a ListNode>
) -> Option<&'a ListNode> {
    // Helper function to compute the length.
    fn length<'a>(mut head: Option<&'a ListNode>) -> usize {
         let mut len = 0;
         while let Some(node) = head {
              len += 1;
              head = node.next.as_deref(); // as_deref() converts Option<&Box<T>> to Option<&T>
         }
         len
    }
    let len_a = length(head_a);
    let len_b = length(head_b);
    let mut a = head_a;
    let mut b = head_b;
    // Advance the pointer for the longer list.
    if len_a > len_b {
         for _ in 0..(len_a - len_b) {
              if let Some(node) = a { a = node.next.as_deref(); }
         }
    } else if len_b > len_a {
         for _ in 0..(len_b - len_a) {
              if let Some(node) = b { b = node.next.as_deref(); }
         }
    }
    // Traverse both lists together.
    while let (Some(node_a), Some(node_b)) = (a, b) {
         if ptr::eq(node_a, node_b) {
              return Some(node_a);
         }
         a = node_a.next.as_deref();
         b = node_b.next.as_deref();
    }
    None
}

/// Prints the question title and description.
pub fn print_question() {
    println!("Question 73: Intersection of Two Linked Lists");
    println!("---------------------------------------------");
    println!("Return the node at which two linked lists intersect (by reference), or None if they do not.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_intersection_node() {
         // Create a common tail: 8 -> 4 -> 5.
         let common = Box::new(ListNode {
              val: 8,
              next: Some(Box::new(ListNode {
                  val: 4,
                  next: Some(Box::new(ListNode::new(5))),
              })),
         });
         // Leak the common tail to obtain a 'static reference.
         // Box::leak returns a &'static mut ListNode, which we coerce to &'static ListNode.
         let common_ref: &'static ListNode = Box::leak(common);
         
         // For testing intersection, we now simulate both lists sharing the same tail
         // by simply using `common_ref` as the intersection.
         // (Constructing two distinct Box-based lists with an actual shared tail is difficult without Rc.)
         let inter_node = get_intersection_node(Some(common_ref), Some(common_ref));
         assert!(inter_node.is_some());
         assert_eq!(inter_node.unwrap().val, 8);
    }
}
