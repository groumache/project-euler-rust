pub mod problem1to10;

// There are 2 tests for p_009 in order to see (in practice) if v2 is actually
// faster than v1 without actually measuring time or anything.
#[cfg(test)]
mod tests {
    use crate::problem1to10::*;
    #[test]
    #[ignore]
    fn test_p001() {
        assert_eq!(p001::v1(10), 23);
        assert_eq!(p001::v2(10), 23);
    }
    #[test]
    #[ignore]
    fn test_p002() {
        assert_eq!(p002::v1(10), 44);
    }
    #[test]
    #[ignore]
    fn test_p003() {
        assert_eq!(p003::v1(13195), 29);
        assert_eq!(p003::v2(13195), 29);
        assert_eq!(p003::v3(13195), 29);
    }
    #[test]
    #[ignore]
    fn test_p004() {
        assert_eq!(p004::v1(2), 9009);
    }
    #[test]
    #[ignore]
    fn test_p005() {
        assert_eq!(p005::v1(10), 2520);
    }
    #[test]
    #[ignore]
    fn test_p006() {
        assert_eq!(p006::v1(10), 2640);
    }
    #[test]
    #[ignore]
    fn test_p007() {
        assert_eq!(p007::v1(6), 13);
    }
    #[test]
    #[ignore]
    fn test_p008() {
        assert_eq!(p008::v1(4), 5832);
    }
    #[test]
    #[ignore]
    fn test_p009_v1() {
        let abc = p009::v1();
        assert_eq!(abc.0.pow(2) + abc.1.pow(2), abc.2.pow(2));
        assert_eq!(abc.0 + abc.1 + abc.2, 1000);
    }
    #[test]
    #[ignore]
    fn test_p009_v2() {
        let abc = p009::v2();
        assert_eq!(abc.0.pow(2) + abc.1.pow(2), abc.2.pow(2));
        assert_eq!(abc.0 + abc.1 + abc.2, 1000);
    }
    #[test]
    fn test_p010() {
        assert_eq!(p010::v1(10), 17);
    }
}
