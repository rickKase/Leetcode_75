/*
Question 12: Maximum Depth of Binary Tree

Question:
---------
Given a binary tree, find its maximum depth.
The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
*/

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

/// Returns the maximum depth of the binary tree.
pub fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => 1 + std::cmp::max(max_depth(node.left), max_depth(node.right)),
    }
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 12: Maximum Depth of Binary Tree");
    println!("------------------------------------------");
    println!("Find the maximum depth (longest root-to-leaf path) of a binary tree.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_depth_empty() {
        assert_eq!(max_depth(None), 0);
    }
    
    #[test]
    fn test_max_depth() {
        // Construct a binary tree:
        //       3
        //      / \
        //     9  20
        //       /  \
        //      15   7
        let mut root = Box::new(TreeNode::new(3));
        root.left = Some(Box::new(TreeNode::new(9)));
        let mut right = Box::new(TreeNode::new(20));
        right.left = Some(Box::new(TreeNode::new(15)));
        right.right = Some(Box::new(TreeNode::new(7)));
        root.right = Some(right);
        
        assert_eq!(max_depth(Some(root)), 3);
    }
}
