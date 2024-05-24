/*
Question 13: Binary Tree Level Order Traversal

Question:
---------
Given a binary tree, return the level order traversal of its nodes' values.
(i.e., from left to right, level by level).
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

/// Returns a vector of vectors, where each inner vector contains the values at that level of the tree.
pub fn level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }
    
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);
    
    while !queue.is_empty() {
        let mut level = Vec::new();
        let size = queue.len();
        for _ in 0..size {
            if let Some(Some(node)) = queue.pop_front() {
                level.push(node.val);
                if node.left.is_some() {
                    queue.push_back(node.left);
                }
                if node.right.is_some() {
                    queue.push_back(node.right);
                }
            }
        }
        result.push(level);
    }
    
    result
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 13: Binary Tree Level Order Traversal");
    println!("----------------------------------------------");
    println!("Return the level order (breadth-first) traversal of a binary tree.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_level_order_empty() {
        let result = level_order(None);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_level_order() {
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
        
        let result = level_order(Some(root));
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(result, expected);
    }
}
