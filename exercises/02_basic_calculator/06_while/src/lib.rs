// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // We pretend that n is going to be 3, and the result should be 6
    let mut result: u32 = 1; // here we initialize the result which will be returned at the end of the function, at the beginning it is always 1 and this is why factorial of 1 will be 1

    let mut i = n; // here we initialize i, the counter, and at the start it is always equal to n, so here it will be 3

    // now we are going to do a while loop

    //we compare i to 1, and we use the > sign. That means that i starts at 3, then it goes to 2, and when it reaches 1 we don't enter the loop

    while i > 1 { // no parenthesis just like the if statement
        result *= i; //here we multiply result by i, so in the first step it is going to be 1 * 3 which is 3
        i -= 1; // here we reduce i by one, so at the beginning i was 3, and now it is 2
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
