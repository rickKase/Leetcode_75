/*
Question 9: Climbing Stairs

Question:
---------
You are climbing a staircase. It takes n steps to reach the top.
Each time you can either climb 1 or 2 steps.
In how many distinct ways can you climb to the top?
*/

/// Returns the number of distinct ways to climb to the top.
pub fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    let mut a = 1;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 9: Climbing Stairs");
    println!("--------------------------");
    println!("You are climbing a staircase. It takes n steps to reach the top.");
    println!("Each time you can either climb 1 or 2 steps.");
    println!("In how many distinct ways can you climb to the top?");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
    }
}
