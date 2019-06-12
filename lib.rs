mod problem001;

// If we list all the natural numbers below 10 that are multiples
// of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
#[cfg(test)]
mod p001 {
    use crate::problem001::prob1;
    #[test]
    fn test_prob_1() {
        assert_eq!(prob1::version1(10), 23);
    }
}
