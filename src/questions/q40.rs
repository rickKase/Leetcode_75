/*
Question 40: Binary Tree Maximum Path Sum

Question:
---------
Given a non-empty binary tree, find the maximum path sum.
A path is defined as any sequence of nodes from some starting node to any node in the tree along the parent-child connections.
The path must contain at least one node and does not need to go through the root.

Example:
---------
Input: root = [-10,9,20,null,null,15,7]
Output: 42
Explanation: The maximum path sum is 15 + 20 + 7 = 42.
*/

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

/// Returns the maximum path sum in the binary tree.
pub fn max_path_sum(root: Option<Box<TreeNode>>) -> i32 {
    fn helper(root: &Option<Box<TreeNode>>, max_sum: &mut i32) -> i32 {
        if let Some(node) = root {
            let left = helper(&node.left, max_sum).max(0);
            let right = helper(&node.right, max_sum).max(0);
            let current = node.val + left + right;
            *max_sum = (*max_sum).max(current);
            node.val + left.max(right)
        } else {
            0
        }
    }
    let mut max_sum = i32::MIN;
    helper(&root, &mut max_sum);
    max_sum
}

pub fn print_question() {
    println!("Question 40: Binary Tree Maximum Path Sum");
    println!("-----------------------------------------");
    println!("Find the maximum path sum in a binary tree.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_path_sum() {
        // Build the tree:
        //       -10
        //       /  \
        //      9   20
        //         /  \
        //        15   7
        let mut root = Box::new(TreeNode::new(-10));
        root.left = Some(Box::new(TreeNode::new(9)));
        let mut node20 = Box::new(TreeNode::new(20));
        node20.left = Some(Box::new(TreeNode::new(15)));
        node20.right = Some(Box::new(TreeNode::new(7)));
        root.right = Some(node20);
        
        assert_eq!(max_path_sum(Some(root)), 42);
    }
}
