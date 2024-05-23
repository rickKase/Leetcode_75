/*
Question 11: Symmetric Tree

Question:
---------
Given a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

Example:
---------
    1
   / \
  2   2
 / \ / \
3  4 4  3
is symmetric.
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

/// Checks if the binary tree is symmetric.
pub fn is_symmetric(root: Option<Box<TreeNode>>) -> bool {
    fn is_mirror(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {
        match (t1, t2) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                n1.val == n2.val &&
                is_mirror(&n1.left, &n2.right) &&
                is_mirror(&n1.right, &n2.left)
            },
            _ => false,
        }
    }
    
    is_mirror(&root, &root)
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 11: Symmetric Tree");
    println!("---------------------------");
    println!("Check whether a binary tree is a mirror of itself (symmetric).");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symmetric() {
        // Construct a symmetric tree:
        //      1
        //     / \
        //    2   2
        //   / \ / \
        //  3  4 4  3
        let mut root = Box::new(TreeNode::new(1));
        let mut left = Box::new(TreeNode::new(2));
        let mut right = Box::new(TreeNode::new(2));
        left.left = Some(Box::new(TreeNode::new(3)));
        left.right = Some(Box::new(TreeNode::new(4)));
        right.left = Some(Box::new(TreeNode::new(4)));
        right.right = Some(Box::new(TreeNode::new(3)));
        root.left = Some(left);
        root.right = Some(right);
        
        assert_eq!(is_symmetric(Some(root)), true);
    }
    
    #[test]
    fn test_not_symmetric() {
        // Construct a non-symmetric tree:
        //    1
        //   / \
        //  2   2
        //   \   \
        //    3   3
        let mut root = Box::new(TreeNode::new(1));
        let mut left = Box::new(TreeNode::new(2));
        let mut right = Box::new(TreeNode::new(2));
        left.right = Some(Box::new(TreeNode::new(3)));
        right.right = Some(Box::new(TreeNode::new(3)));
        root.left = Some(left);
        root.right = Some(right);
        
        assert_eq!(is_symmetric(Some(root)), false);
    }
}
