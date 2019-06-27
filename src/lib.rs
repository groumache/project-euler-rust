pub mod problem001to010;
pub mod problem011to020;
pub mod problem021to030;
pub mod problem031to040;
pub mod problem041to050;

// there are 2 tests for p009: v1 & v2
#[cfg(test)]
mod tests_1to10 {
    use crate::problem001to010::*;
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
        assert_eq!(p003::v4(13195), 29);
        assert_eq!(p003::v5(13195), 29);
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
    #[ignore]
    fn test_p010() {
        assert_eq!(p010::v1(10), 17);
    }
}

// No test for p011, p013, p018, p019 because there aren't any example in the problem statements.
#[cfg(test)]
mod tests_11to20 {
    use crate::problem011to020::*;
    #[test]
    #[ignore]
    fn test_p012() {
        assert_eq!(p012::v1(5), 28);
    }
    // NOTE: I made the following test without any example from the problem statement.
    #[test]
    #[ignore]
    fn test_p014() {
        assert_eq!(p014::v1(10), 9);
    }
    #[test]
    #[ignore]
    fn test_p015() {
        assert_eq!(p015::v1(2), 6);
    }
    #[test]
    #[ignore]
    fn test_p016() {
        assert_eq!(p016::v1(15), 26);
    }
    #[test]
    #[ignore]
    fn test_p017() {
        assert_eq!(p017::v1(5), 19);
    }
    #[test]
    #[ignore]
    fn test_p020() {
        assert_eq!(p020::v1(10), 27);
    }
}
