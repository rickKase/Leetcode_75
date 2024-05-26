/*
Question 25: Coin Change

Question:
---------
You are given coins of different denominations and a total amount of money.
Write a function to compute the fewest number of coins that you need to make up that amount.
If that amount of money cannot be made up by any combination of the coins, return -1.

Example:
---------
Input: coins = [1, 2, 5], amount = 11
Output: 3
Explanation: 11 = 5 + 5 + 1
*/

use std::cmp::min;

/// Returns the minimum number of coins needed to make up the amount, or -1 if not possible.
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![amount as i32 + 1; amount + 1];
    dp[0] = 0;
    for i in 1..=amount {
        for &coin in &coins {
            if (coin as usize) <= i {
                dp[i] = min(dp[i], dp[i - coin as usize] + 1);
            }
        }
    }
    if dp[amount] > amount as i32 {
        -1
    } else {
        dp[amount]
    }
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 25: Coin Change");
    println!("------------------------");
    println!("Compute the fewest number of coins needed to make up a given amount.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
    }
}
