/*
Question 14: Same Tree

Question:
---------
Given two binary trees, write a function to check if they are the same or not.
Two binary trees are considered the same if they are structurally identical and the nodes have the same value.
*/

#[derive(Debug, PartialEq, Eq)]
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

/// Checks if two binary trees are the same.
pub fn is_same_tree(p: Option<Box<TreeNode>>, q: Option<Box<TreeNode>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(n1), Some(n2)) => {
            n1.val == n2.val &&
            is_same_tree(n1.left, n2.left) &&
            is_same_tree(n1.right, n2.right)
        },
        _ => false,
    }
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 14: Same Tree");
    println!("----------------------");
    println!("Check if two binary trees are structurally identical and have the same node values.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_same_tree_true() {
        // Create two identical trees:
        //     1
        //    / \
        //   2   3
        let mut tree1 = Box::new(TreeNode::new(1));
        tree1.left = Some(Box::new(TreeNode::new(2)));
        tree1.right = Some(Box::new(TreeNode::new(3)));
        
        let mut tree2 = Box::new(TreeNode::new(1));
        tree2.left = Some(Box::new(TreeNode::new(2)));
        tree2.right = Some(Box::new(TreeNode::new(3)));
        
        assert_eq!(is_same_tree(Some(tree1), Some(tree2)), true);
    }
    
    #[test]
    fn test_same_tree_false() {
        // Create two different trees:
        // Tree 1: 1 -> 2
        // Tree 2: 1 -> null -> 2
        let tree1 = Box::new(TreeNode::new(1));
        let mut tree2 = Box::new(TreeNode::new(1));
        tree2.right = Some(Box::new(TreeNode::new(2)));
        
        assert_eq!(is_same_tree(Some(tree1), Some(tree2)), false);
    }
}
