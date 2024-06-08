/*
Question 47: Word Ladder

Question:
---------
Given two words (beginWord and endWord), and a dictionary's word list, find the length of the shortest transformation sequence from beginWord to endWord, such that:
  - Only one letter can be changed at a time.
  - Each transformed word must exist in the word list.
Return 0 if no such transformation sequence exists.

Example:
---------
Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
Output: 5
Explanation: One shortest transformation is "hit" -> "hot" -> "dot" -> "dog" -> "cog", which is 5 words long.
*/

use std::collections::{HashSet, VecDeque};

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let word_set: HashSet<String> = word_list.into_iter().collect();
    if !word_set.contains(&end_word) {
        return 0;
    }
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(begin_word.clone());
    visited.insert(begin_word);
    let mut level = 1;
    
    while !queue.is_empty() {
        let level_size = queue.len();
        for _ in 0..level_size {
            let current = queue.pop_front().unwrap();
            if current == end_word {
                return level;
            }
            for i in 0..current.len() {
                let mut current_chars: Vec<char> = current.chars().collect();
                for ch in 'a' as u8..='z' as u8 {
                    let c = ch as char;
                    if current_chars[i] == c {
                        continue;
                    }
                    current_chars[i] = c;
                    let new_word: String = current_chars.iter().collect();
                    if word_set.contains(&new_word) && !visited.contains(&new_word) {
                        visited.insert(new_word.clone());
                        queue.push_back(new_word);
                    }
                }
            }
        }
        level += 1;
    }
    0
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 47: Word Ladder");
    println!("------------------------");
    println!("Find the length of the shortest transformation sequence from beginWord to endWord,");
    println!("changing only one letter at a time and using only words from the given list.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ladder_length() {
        let begin = "hit".to_string();
        let end = "cog".to_string();
        let word_list = vec!["hot","dot","dog","lot","log","cog"]
            .into_iter().map(String::from).collect();
        assert_eq!(ladder_length(begin, end, word_list), 5);
    }
    
    #[test]
    fn test_no_ladder() {
        let begin = "hit".to_string();
        let end = "cog".to_string();
        let word_list = vec!["hot","dot","dog","lot","log"]
            .into_iter().map(String::from).collect();
        assert_eq!(ladder_length(begin, end, word_list), 0);
    }
}
