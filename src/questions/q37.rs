/*
Question 37: Clone Graph

Question:
---------
Given a reference of a node in a connected undirected graph, return a deep copy (clone) of the graph.
Each node contains a value and a list of its neighbors.

Example:
---------
Input: Node 1 connected to Node 2, and Node 2 connected back to Node 1.
Output: A deep copy of the graph.
*/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    /// Creates a new graph node wrapped in Rc<RefCell<>>
    pub fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, neighbors: vec![] }))
    }
}

/// Clones the given graph using depth-first search.
pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    if node.is_none() {
        return None;
    }
    let node = node.unwrap();
    let mut map = HashMap::new();
    fn dfs(node: Rc<RefCell<Node>>, map: &mut HashMap<i32, Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
        let val = node.borrow().val;
        if let Some(cloned) = map.get(&val) {
            return cloned.clone();
        }
        let clone = Rc::new(RefCell::new(Node { val, neighbors: vec![] }));
        map.insert(val, clone.clone());
        for neighbor in node.borrow().neighbors.iter() {
            let cloned_neighbor = dfs(neighbor.clone(), map);
            clone.borrow_mut().neighbors.push(cloned_neighbor);
        }
        clone
    }
    Some(dfs(node, &mut map))
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 37: Clone Graph");
    println!("------------------------");
    println!("Clone an undirected graph given a reference node.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clone_graph() {
        // Create a simple graph: Node 1 <-> Node 2.
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        node1.borrow_mut().neighbors.push(node2.clone());
        node2.borrow_mut().neighbors.push(node1.clone());

        let cloned = clone_graph(Some(node1.clone()));
        assert!(cloned.is_some());
        let cloned = cloned.unwrap();
        assert_eq!(cloned.borrow().val, 1);
        assert_eq!(cloned.borrow().neighbors.len(), 1);
        let neighbor = cloned.borrow().neighbors[0].clone();
        assert_eq!(neighbor.borrow().val, 2);
    }
}
