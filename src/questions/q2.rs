/*
Leetcode Problem 2: Best Time to Buy and Sell Stock

Question:
---------
You are given an array `prices` where `prices[i]` is the price of a given stock on the i-th day.
Find the maximum profit you can achieve by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
If no profit can be achieved, return 0.
*/

/// Returns the maximum profit that can be achieved.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;
    for price in prices {
        if price < min_price {
            min_price = price;
        } else {
            let profit = price - min_price;
            if profit > max_profit {
                max_profit = profit;
            }
        }
    }
    max_profit
}

/// Prints the question to the CLI.
pub fn print_question() {
    println!("Leetcode Problem 2: Best Time to Buy and Sell Stock");
    println!("-----------------------------------------------------");
    println!("Given an array `prices` where `prices[i]` is the price of a given stock on the i-th day,");
    println!("find the maximum profit you can achieve by choosing a single day to buy and a different day in the future to sell.");
    println!("If no profit can be achieved, return 0.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
