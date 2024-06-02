/*
Question 39: Lowest Common Ancestor of a Binary Tree

Question:
---------
Given a binary tree and two node values p and q, return the lowest common ancestor (LCA) of these nodes.
The LCA is defined as the deepest node that has both p and q as descendants.

Example:
---------
Input: root of the binary tree and p = 5, q = 1 (in the tree below)
         3
        / \
       5   1
      / \ / \
     6  2 0  8
        / \
       7   4
Output: Node with value 3
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

/// Returns a reference to the lowest common ancestor of nodes with values p and q.
/// (Uses references to avoid taking ownership of the tree.)
pub fn lowest_common_ancestor<'a>(
    root: Option<&'a Box<TreeNode>>,
    p: i32,
    q: i32
) -> Option<&'a Box<TreeNode>> {
    if let Some(node) = root {
        if node.val == p || node.val == q {
            return Some(node);
        }
        let left = lowest_common_ancestor(node.left.as_ref(), p, q);
        let right = lowest_common_ancestor(node.right.as_ref(), p, q);
        if left.is_some() && right.is_some() {
            return Some(node);
        }
        left.or(right)
    } else {
        None
    }
}

pub fn print_question() {
    println!("Question 39: Lowest Common Ancestor of a Binary Tree");
    println!("------------------------------------------------------");
    println!("Given a binary tree and two node values, find their lowest common ancestor (LCA).");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lowest_common_ancestor() {
        // Build the following tree:
        //       3
        //      / \
        //     5   1
        //    / \ / \
        //   6  2 0  8
        //     / \
        //    7   4
        let mut root = Box::new(TreeNode::new(3));
        let mut node5 = Box::new(TreeNode::new(5));
        let mut node1 = Box::new(TreeNode::new(1));
        node5.left = Some(Box::new(TreeNode::new(6)));
        let mut node2 = Box::new(TreeNode::new(2));
        node2.left = Some(Box::new(TreeNode::new(7)));
        node2.right = Some(Box::new(TreeNode::new(4)));
        node5.right = Some(node2);
        node1.left = Some(Box::new(TreeNode::new(0)));
        node1.right = Some(Box::new(TreeNode::new(8)));
        root.left = Some(node5);
        root.right = Some(node1);

        let lca = lowest_common_ancestor(Some(&root), 5, 1);
        assert!(lca.is_some());
        assert_eq!(lca.unwrap().val, 3);

        let lca2 = lowest_common_ancestor(Some(&root), 7, 4);
        assert!(lca2.is_some());
        assert_eq!(lca2.unwrap().val, 2);
    }
}
