// TODO:
//  1. do not modify the vector received as argument in 'no_double'
//  2. make 'no_double' generic (no_double<T>(v: & Vec<T>))


pub mod useful_func;
pub mod problem001to010;
pub mod problem011to020;
pub mod problem021to030;
pub mod problem031to040;
pub mod problem041to050;

#[cfg(test)]
mod test_useful_func {
    use crate::useful_func::*;
    mod prime_numbers_tests {
        use super::prime_numbers::*;
        #[test]
        #[ignore]
        fn test_is_prime() {
            assert_eq!(is_prime(0), false);
            assert_eq!(is_prime(1), false);
            assert_eq!(is_prime(2), true);
            assert_eq!(is_prime(3), true);
            assert_eq!(is_prime(4), false);
            assert_eq!(is_prime(5), true);
            assert_eq!(is_prime(17), true);
        }
        #[test]
        #[ignore]
        fn test_primes_below() {
            assert_eq!(primes_below(2), [].to_vec());
            assert_eq!(primes_below(3), [2].to_vec());
            assert_eq!(primes_below(10), [2,3,5,7].to_vec());
        }
        #[test]
        #[ignore]
        fn test_primes_iterator1() {
            let min = 0;
            let max = 0;
            let mut counter = 0;
            for _ in primes_minmax(min, max) {
                counter += 1;
            }
            assert_eq!(counter, 0);
        }
        #[test]
        #[ignore]
        fn test_primes_iterator2() {
            let min = 0;
            let max = 10;
            let mut primes: Vec<u32> = Vec::new();
            for p in primes_minmax(min, max) {
                primes.push(p);
            }
            assert_eq!(primes, [2,3,5,7].to_vec());
        }
        #[test]
        #[ignore]
        fn test_primes_iterator3() {
            let min = 0;
            let max = 10;
            let mut counter = 0;
            for p in primes_minmax(min, max).rev() {
                counter += 1;
                assert!(
                    match counter {
                        1 => p == 7,
                        2 => p == 5,
                        3 => p == 3,
                        4 => p == 2,
                        _ => false,
                    }
                )
            }
        }
        #[test]
        #[ignore]
        fn test_prime_factors() {
            assert_eq!(prime_factors(2), [2].to_vec());
            assert_eq!(prime_factors(7), [7].to_vec());
            assert_eq!(prime_factors(12), [2,3].to_vec());
        }
    }
    mod digits_numbers_tests {
        use super::digits_numbers::*;
        #[test]
        #[ignore]
        fn test_get_digits() {
            assert_eq!(get_digits(58), [8,5].to_vec());
            assert_eq!(get_digits(0), [0].to_vec());
        }
        #[test]
        #[ignore]
        fn test_get_number() {
            assert_eq!(get_number(& [7,5,3].to_vec()), 357);
            assert_eq!(get_number(& [0].to_vec()), 0);
        }
        #[test]
        #[ignore]
        fn test_no_double() {
            assert!(no_double(&mut [5,7,8].to_vec()));
            assert!(no_double(&mut [0].to_vec()));
            assert!(!no_double(&mut [5,5].to_vec()));
            assert!(!no_double(&mut [4,8,4].to_vec()));
        }
    }
    mod other_func_tests {
        use super::other_func::*;
        #[test]
        #[ignore]
        fn test_triangle_iterator() {
            let mut v: Vec<u32> = Vec::new();
            for i in triangle_inf() {
                if i > 10 { break; }
                v.push(i);
            }
            assert_eq!(v, [1,3,6,10].to_vec());
        }
        #[test]
        #[ignore]
        fn test_pentagon_inf() {
            let mut v: Vec<u32> = Vec::new();
            for i in pentagon_inf() {
                if i > 10 { break; }
                v.push(i);
            }
            assert_eq!(v, [1,5].to_vec());
        }
        #[test]
        #[ignore]
        fn test_is_pentagon() {
            assert!(is_pentagon(1));
            assert!(is_pentagon(12));
        }
        #[test]
        #[ignore]
        fn test_hexagon_inf() {
            let mut v: Vec<u32> = Vec::new();
            for i in hexagon_inf() {
                if i > 10 { break; }
                v.push(i);
            }
            assert_eq!(v, [1,6].to_vec());
        }
        #[test]
        #[ignore]
        fn test_is_hexagon() {
            assert!(is_hexagon(1));
            assert!(is_hexagon(15));
        }
    }
}

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
