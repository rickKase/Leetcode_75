/*
Question 15: Invert Binary Tree

Question:
---------
Invert a binary tree.

Example:
---------
Input:
     4
   /   \
  2     7
 / \   / \
1   3 6   9

Output:
     4
   /   \
  7     2
 / \   / \
9   6 3   1
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

/// Inverts the binary tree.
pub fn invert_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    match root {
        None => None,
        Some(mut node) => {
            let left_inverted = invert_tree(node.left.take());
            let right_inverted = invert_tree(node.right.take());
            node.left = right_inverted;
            node.right = left_inverted;
            Some(node)
        }
    }
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 15: Invert Binary Tree");
    println!("-------------------------------");
    println!("Invert a binary tree (swap every left and right child).");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invert_tree() {
        // Construct the binary tree:
        //      4
        //     / \
        //    2   7
        //   / \ / \
        //  1  3 6  9
        let mut root = Box::new(TreeNode::new(4));
        let mut left = Box::new(TreeNode::new(2));
        let mut right = Box::new(TreeNode::new(7));
        left.left = Some(Box::new(TreeNode::new(1)));
        left.right = Some(Box::new(TreeNode::new(3)));
        right.left = Some(Box::new(TreeNode::new(6)));
        right.right = Some(Box::new(TreeNode::new(9)));
        root.left = Some(left);
        root.right = Some(right);
        
        let inverted = invert_tree(Some(root));
        
        // The inverted tree should be:
        //      4
        //     / \
        //    7   2
        //   / \ / \
        //  9  6 3  1
        if let Some(node) = inverted {
            assert_eq!(node.val, 4);
            if let (Some(left_node), Some(right_node)) = (node.left, node.right) {
                assert_eq!(left_node.val, 7);
                assert_eq!(right_node.val, 2);
            } else {
                panic!("Inversion failed: missing children.");
            }
        } else {
            panic!("Inversion failed: tree is None.");
        }
    }
}
