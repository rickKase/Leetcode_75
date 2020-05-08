/*
Question 6: Merge Two Sorted Lists

Question:
---------
Merge two sorted linked lists and return it as a sorted list.
The list should be made by splicing together the nodes of the first two lists.

Note:
The linked list is represented as Option<Box<ListNode>>.
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

/// Merges two sorted linked lists and returns the merged list.
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(mut n1), Some(mut n2)) => {
            if n1.val < n2.val {
                let next = n1.next.take();
                n1.next = merge_two_lists(next, Some(n2));
                Some(n1)
            } else {
                let next = n2.next.take();
                n2.next = merge_two_lists(Some(n1), next);
                Some(n2)
            }
        },
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        (None, None) => None,
    }
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 6: Merge Two Sorted Lists");
    println!("----------------------------------");
    println!("Merge two sorted linked lists and return it as a sorted list.");
    println!("The list should be made by splicing together the nodes of the first two lists.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function: convert vector to linked list.
    fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for num in nums.into_iter().rev() {
            let mut node = Box::new(ListNode::new(num));
            node.next = current;
            current = Some(node);
        }
        current
    }

    // Helper function: convert linked list to vector.
    fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    #[test]
    fn test_merge_two_lists() {
        let l1 = vec_to_list(vec![1, 2, 4]);
        let l2 = vec_to_list(vec![1, 3, 4]);
        let merged = merge_two_lists(l1, l2);
        assert_eq!(list_to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
    }
}
