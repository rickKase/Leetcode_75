/*
Question 38: Serialize and Deserialize Binary Tree

Question:
---------
Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm works.
You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.

Example:
---------
Input: A binary tree:
     1
    / \
   2   3
      / \
     4   5
Output: A string representation that can be deserialized back into the same tree.
*/

use std::str::FromStr;

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

pub struct Codec;

impl Codec {
    /// Serializes a binary tree to a string using pre-order traversal.
    pub fn serialize(root: Option<Box<TreeNode>>) -> String {
        fn helper(root: &Option<Box<TreeNode>>, s: &mut String) {
            if let Some(node) = root {
                s.push_str(&node.val.to_string());
                s.push(' ');
                helper(&node.left, s);
                helper(&node.right, s);
            } else {
                s.push_str("# ");
            }
        }
        let mut s = String::new();
        helper(&root, &mut s);
        s.trim_end().to_string()
    }

    /// Deserializes a string to reconstruct the binary tree.
    pub fn deserialize(data: String) -> Option<Box<TreeNode>> {
        // Use split_whitespace() directly to get an iterator yielding &str.
        fn helper<'a, I>(iter: &mut I) -> Option<Box<TreeNode>>
        where
            I: Iterator<Item = &'a str>,
        {
            if let Some(val) = iter.next() {
                if val == "#" {
                    return None;
                }
                let num = i32::from_str(val).ok()?;
                let mut node = Box::new(TreeNode::new(num));
                node.left = helper(iter);
                node.right = helper(iter);
                Some(node)
            } else {
                None
            }
        }
        let mut iter = data.split_whitespace();
        helper(&mut iter)
    }
}

pub fn print_question() {
    println!("Question 38: Serialize and Deserialize Binary Tree");
    println!("---------------------------------------------------");
    println!("Design an algorithm to convert a binary tree to a string and back.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        // Create a tree: 
        //     1
        //    / \
        //   2   3
        //      / \
        //     4   5
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        let mut right = Box::new(TreeNode::new(3));
        right.left = Some(Box::new(TreeNode::new(4)));
        right.right = Some(Box::new(TreeNode::new(5)));
        root.right = Some(right);

        let serialized = Codec::serialize(Some(root));
        let deserialized = Codec::deserialize(serialized);
        // For testing, serialize the deserialized tree and compare.
        let reserialized = Codec::serialize(deserialized);
        assert_eq!(reserialized, "1 2 # # 3 4 # # 5 # #");
    }
}
