/*
Question 36: Linked List Cycle II

Question:
---------
Given a linked list, return the node where the cycle begins. If there is no cycle, return None.

Example:
---------
Input: head = [3,2,0,-4] with a cycle connecting the tail to the node with value 2
Output: Node with value 2
*/

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    /// Creates a new ListNode wrapped in Rc<RefCell<>>
    pub fn new(val: i32) -> Rc<RefCell<ListNode>> {
        Rc::new(RefCell::new(ListNode { val, next: None }))
    }
}

/// Returns the node where the cycle begins (if any) using Floyd's Tortoise and Hare algorithm.
pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    let mut slow = head.clone();
    let mut fast = head.clone();
    while let (Some(s), Some(f)) = (slow.clone(), fast.clone()) {
        // Move slow one step.
        slow = s.borrow().next.clone();
        // Move fast two steps.
        if let Some(next_fast) = f.borrow().next.clone() {
            fast = next_fast.borrow().next.clone();
        } else {
            return None;
        }
        if let (Some(s_node), Some(f_node)) = (slow.clone(), fast.clone()) {
            if Rc::ptr_eq(&s_node, &f_node) {
                // Cycle detected; now find the entry point.
                let mut entry = head.clone();
                while let (Some(e), Some(s)) = (entry.clone(), slow.clone()) {
                    if Rc::ptr_eq(&e, &s) {
                        return Some(e);
                    }
                    entry = e.borrow().next.clone();
                    slow = s.borrow().next.clone();
                }
            }
        }
    }
    None
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 36: Linked List Cycle II");
    println!("---------------------------------");
    println!("Return the node where the cycle begins in a linked list (or None if no cycle exists).");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_detect_cycle() {
        // Create a list: 3 -> 2 -> 0 -> -4, with a cycle starting at node with value 2.
        let node1 = ListNode::new(3);
        let node2 = ListNode::new(2);
        let node3 = ListNode::new(0);
        let node4 = ListNode::new(-4);
        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node4.clone());
        node4.borrow_mut().next = Some(node2.clone()); // cycle here

        let cycle_node = detect_cycle(Some(node1.clone()));
        assert!(cycle_node.is_some());
        let cycle_node = cycle_node.unwrap();
        assert_eq!(cycle_node.borrow().val, 2);
    }

    #[test]
    fn test_no_cycle() {
        let node1 = ListNode::new(1);
        let node2 = ListNode::new(2);
        node1.borrow_mut().next = Some(node2.clone());
        let cycle_node = detect_cycle(Some(node1));
        assert!(cycle_node.is_none());
    }
}
