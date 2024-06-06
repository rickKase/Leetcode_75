/*
Question 42: Reverse Nodes in k-Group

Question:
---------
Given a linked list, reverse the nodes of the list k at a time, and return its modified list.
If the number of nodes is not a multiple of k then left-out nodes in the end should remain as-is.

Example:
---------
Input: head = [1,2,3,4,5], k = 2
Output: [2,1,4,3,5]
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

/// Helper function that reverses the first `k` nodes of the list.
/// It assumes that the list has at least `k` nodes.
/// Returns a tuple `(reversed_head, remainder)`, where:
/// - `reversed_head` is the head of the reversed segment,
/// - `remainder` is the list starting after the first `k` nodes.
fn reverse_first_k(mut head: Option<Box<ListNode>>, k: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut prev = None;
    let mut current = head;
    let mut count = 0;
    while count < k {
         // Since we checked that there are at least k nodes, unwrap is safe here.
         let mut curr_node = current.unwrap();
         let next = curr_node.next.take();
         curr_node.next = prev;
         prev = Some(curr_node);
         current = next;
         count += 1;
    }
    (prev, current)
}

/// Reverses nodes of the linked list in groups of k.
/// If there are fewer than k nodes remaining, those nodes are left as-is.
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let k_usize = k as usize;
    // Check if there are at least k nodes in the list.
    let mut count = 0;
    let mut ptr = &head;
    while let Some(node) = ptr {
         count += 1;
         ptr = &node.next;
    }
    if count < k_usize {
         return head;
    }
    // Reverse the first k nodes.
    let (mut reversed, remainder) = reverse_first_k(head, k_usize);
    // Find the tail of the reversed segment.
    {
         let mut tail = reversed.as_mut().unwrap();
         while let Some(ref mut next) = tail.next {
              tail = next;
         }
         // Recursively process the remainder and attach it.
         tail.next = reverse_k_group(remainder, k);
    }
    reversed
}

/// Prints the question title and description to the CLI.
pub fn print_question() {
    println!("Question 42: Reverse Nodes in k-Group");
    println!("-------------------------------------");
    println!("Given a linked list, reverse the nodes of the list k at a time,");
    println!("and return its modified list. If the number of nodes is not a multiple");
    println!("of k then the left-out nodes in the end should remain as-is.");
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
    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
         let mut vec = Vec::new();
         let mut current = head;
         while let Some(node) = current {
              vec.push(node.val);
              current = node.next;
         }
         vec
    }

    #[test]
    fn test_reverse_k_group_2() {
         let list = vec_to_list(vec![1, 2, 3, 4, 5]);
         let reversed = reverse_k_group(list, 2);
         assert_eq!(list_to_vec(reversed), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn test_reverse_k_group_3() {
         let list = vec_to_list(vec![1, 2, 3, 4, 5]);
         let reversed = reverse_k_group(list, 3);
         assert_eq!(list_to_vec(reversed), vec![3, 2, 1, 4, 5]);
    }
}
