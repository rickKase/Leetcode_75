/*
Question 10: Linked List Cycle

Question:
---------
Given a linked list, determine if it has a cycle in it.
For this problem, we assume the linked list is represented using Rc<RefCell<ListNode>>
to allow cycle creation.
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

/// Returns true if the linked list has a cycle using the fast and slow pointer approach.
pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut slow = head.clone();
    let mut fast = head.clone();
    
    while let (Some(f), Some(s)) = (fast.clone(), slow.clone()) {
        // Advance slow by one step.
        slow = s.borrow().next.clone();
        // Advance fast by two steps.
        fast = match f.borrow().next.clone() {
            Some(next) => next.borrow().next.clone(),
            None => return false,
        };
        
        if let (Some(s_ptr), Some(f_ptr)) = (slow.as_ref(), fast.as_ref()) {
            if Rc::ptr_eq(s_ptr, f_ptr) {
                return true;
            }
        } else {
            return false;
        }
    }
    false
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 10: Linked List Cycle");
    println!("------------------------------");
    println!("Given a linked list, determine if it has a cycle in it.");
    println!("For this problem, we assume the linked list is represented using Rc<RefCell<ListNode>>.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cycle_false() {
        // Create a list: 1 -> 2 -> 3 with no cycle.
        let node1 = ListNode::new(1);
        let node2 = ListNode::new(2);
        let node3 = ListNode::new(3);
        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        assert_eq!(has_cycle(Some(node1)), false);
    }

    #[test]
    fn test_has_cycle_true() {
        // Create a list with a cycle: 1 -> 2 -> 3 -> 2 (cycle)
        let node1 = ListNode::new(1);
        let node2 = ListNode::new(2);
        let node3 = ListNode::new(3);
        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node2.clone()); // cycle here
        assert_eq!(has_cycle(Some(node1)), true);
    }
}
