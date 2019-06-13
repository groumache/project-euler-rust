pub mod problem1to10;

// If we list all the natural numbers below 10 that are multiples
// of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
#[cfg(test)]
mod tests {
    use crate::problem1to10::*;
    #[test]
    fn test_p001() {
        assert_eq!(p001::v1(10), 23);
        assert_eq!(p001::v2(10), 23);
    }
    #[test]
    fn test_p002() {
        assert_eq!(p002::v1(10), 44);
    }
    #[test]
    fn test_p003() {
        assert_eq!(p003::v1(13195), 29);
        assert_eq!(p003::v2(13195), 29);
        assert_eq!(p003::v3(13195), 29);
    }
    #[test]
    fn test_p004() {
        assert_eq!(p004::v1(2), 9009);
    }
    #[test]
    fn test_p005() {
        assert_eq!(p005::v1(10), 2520);
    }
    #[test]
    fn test_p006() {
        assert_eq!(p006::v1(10), 2640);
    }
    #[test]
    fn test_p007() {
        assert_eq!(p007::v1(6), 13);
    }
}
