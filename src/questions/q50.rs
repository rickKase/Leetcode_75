/*
Question 50: Implement Trie (Prefix Tree)

Question:
---------
Implement a trie with insert, search, and startsWith methods.
- insert(word): Inserts the word into the trie.
- search(word): Returns true if the word is in the trie.
- startsWith(prefix): Returns true if there is any word in the trie that starts with the given prefix.

Example:
---------
Trie trie = new Trie();
trie.insert("apple");
trie.search("apple");   // returns true
trie.search("app");     // returns false
trie.startsWith("app"); // returns true
trie.insert("app");
trie.search("app");     // returns true
*/

use std::collections::HashMap;

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Creates a new Trie.
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }
    
    /// Inserts a word into the trie.
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::new());
        }
        node.is_end = true;
    }
    
    /// Returns true if the word is in the trie.
    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            if let Some(next) = node.children.get(&ch) {
                node = next;
            } else {
                return false;
            }
        }
        node.is_end
    }
    
    /// Returns true if there is any word in the trie that starts with the given prefix.
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            if let Some(next) = node.children.get(&ch) {
                node = next;
            } else {
                return false;
            }
        }
        true
    }
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 50: Implement Trie (Prefix Tree)");
    println!("-----------------------------------------");
    println!("Design a Trie with insert, search, and startsWith methods.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
    }
}
