// iterators4.rs

fn inner_factorial(num: u64, total: u64) -> u64 {
    if num == 0 {
        total
    } else {
        inner_factorial(num - 1, total * num)
    }
}

pub fn factorial(num: u64) -> u64 {
    inner_factorial(num, 1)

    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    
    // Here's what they were looking for:
    // (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
