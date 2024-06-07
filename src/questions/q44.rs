/*
Question 44: Find Median from Data Stream

Question:
---------
Design a data structure that supports adding numbers from a data stream and finding the median of all added numbers.
Implement the MedianFinder class.

Example:
---------
addNum(1)
addNum(2)
findMedian() -> 1.5
addNum(3)
findMedian() -> 2.0
*/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct MedianFinder {
    lo: BinaryHeap<i32>,            // Max-heap for the lower half
    hi: BinaryHeap<Reverse<i32>>,     // Min-heap for the higher half
}

impl MedianFinder {
    /// Creates a new MedianFinder.
    pub fn new() -> Self {
        MedianFinder { lo: BinaryHeap::new(), hi: BinaryHeap::new() }
    }
    
    /// Adds a number into the data structure.
    pub fn add_num(&mut self, num: i32) {
        self.lo.push(num);
        // Balance: push the largest of lo into hi.
        if let Some(&top) = self.lo.peek() {
            self.hi.push(Reverse(top));
            self.lo.pop();
        }
        // Maintain size property.
        if self.lo.len() < self.hi.len() {
            if let Some(Reverse(num)) = self.hi.pop() {
                self.lo.push(num);
            }
        }
    }
    
    /// Returns the median of all elements so far.
    pub fn find_median(&self) -> f64 {
        if self.lo.len() > self.hi.len() {
            *self.lo.peek().unwrap() as f64
        } else {
            (*self.lo.peek().unwrap() as f64 + self.hi.peek().unwrap().0 as f64) / 2.0
        }
    }
}

pub fn print_question() {
    println!("Question 44: Find Median from Data Stream");
    println!("-----------------------------------------");
    println!("Design a data structure to add numbers and find the median in a stream.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_median_finder() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }
}
