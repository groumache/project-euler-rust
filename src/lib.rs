pub mod problem1to10;

// If we list all the natural numbers below 10 that are multiples
// of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
#[cfg(test)]
mod tests {
    use crate::problem1to10::p001;
    #[test]
    fn test_prob_1() {
        assert_eq!(p001::version1(10), 23);
        assert_eq!(p001::version2(10), 23);
    }
}
