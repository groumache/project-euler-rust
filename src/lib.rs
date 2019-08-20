pub mod problem001to010;
pub mod problem011to020;
pub mod problem021to030;
pub mod problem031to040;
pub mod problem041to050;
pub mod useful_func;

#[cfg(test)]
mod test_useful_func {
    use crate::useful_func::*;

    mod tests_prime_numbers {
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
            assert_eq!(primes_below(10), [2, 3, 5, 7].to_vec());
        }

        #[test]
        #[ignore]
        fn test_prime_factors() {
            assert_eq!(prime_factors(2), [2].to_vec());
            assert_eq!(prime_factors(7), [7].to_vec());
            assert_eq!(prime_factors(12), [2, 3].to_vec());
        }

        #[test]
        #[ignore]
        fn test_primes_range() {
            let counter = primes_range(0, 0).count();
            assert_eq!(counter, 0);

            let primes: Vec<u32> = primes_range(0, 10).collect();
            assert_eq!(primes, [2, 3, 5, 7].to_vec());
        }

        #[test]
        #[ignore]
        fn test_primes_range_rev() {
            let primes: Vec<u32> = primes_range(0, 10).rev().collect();
            assert_eq!(primes, [7, 5, 3, 2].to_vec());
        }
    }

    mod tests_fibonacci {
        use super::fibonacci::*;

        #[test]
        #[ignore]
        fn test_is_fibonacci() {
            assert!(is_fibonacci(1));
            assert!(is_fibonacci(2));
            assert!(is_fibonacci(3));
            assert!(is_fibonacci(5));
        }

        #[test]
        #[ignore]
        fn test_fibonacci_range() {
            let fib: Vec<u32> = fibonacci_range(0, 10).collect();
            assert_eq!(fib, [1,2,3,5,8].to_vec());
        }
    }

    mod tests_digits {
        use super::digits::*;

        #[test]
        #[ignore]
        fn test_num_to_digits() {
            assert_eq!(num_to_digits(58), [8, 5].to_vec());
            assert_eq!(num_to_digits(0), [0].to_vec());
        }

        #[test]
        #[ignore]
        fn test_digits_to_num() {
            assert_eq!(digits_to_num(&[7, 5, 3].to_vec()), 357);
            assert_eq!(digits_to_num(&[0].to_vec()), 0);
        }

        #[test]
        #[ignore]
        fn test_no_double() {
            assert!(no_double(&mut [5, 7, 8].to_vec()));
            assert!(no_double(&mut [0].to_vec()));
            assert!(! no_double(&mut [5, 5].to_vec()));
            assert!(! no_double(&mut [4, 8, 4].to_vec()));
        }
    }

    mod tests_triangle_num {
        use super::triangle_num::*;

        #[test]
        #[ignore]
        fn test_is_triangle() {
            assert!(is_triangle(1));
            assert!(is_triangle(15));
        }

        #[test]
        #[ignore]
        fn test_triangles_range() {
            let v: Vec<u32> = triangles_range(0, 11).collect();
            assert_eq!(v, [1, 3, 6, 10].to_vec());
        }
    }

    mod tests_pentagonal_num {
        use super::pentagonal_num::*;

        #[test]
        #[ignore]
        fn test_is_pentagon() {
            assert!(is_pentagon(1));
            assert!(is_pentagon(12));
        }

        #[test]
        #[ignore]
        fn test_pentagons_range() {
            let v: Vec<u32> = pentagons_range(0, 11).collect();
            assert_eq!(v, [1, 5].to_vec());
        }
    }

    mod tests_hexagonal_num {
        use super::hexagonal_num::*;

        #[test]
        #[ignore]
        fn test_is_hexagon() {
            assert!(is_hexagon(1));
            assert!(is_hexagon(15));
        }

        #[test]
        #[ignore]
        fn test_hexagons_range() {
            let v: Vec<u32> = hexagons_range(0, 11).collect();
            assert_eq!(v, [1, 6].to_vec());
        }
    }

    mod tests_other_func {
        use super::other_func::*;

        #[test]
        #[ignore]
        fn test_is_square() {
            assert!(! is_square(0));
            assert!(! is_square(2));

            assert!(is_square(1));
            assert!(is_square(4));
        }

        #[test]
        #[ignore]
        fn test_factors() {
            assert_eq!(factors(5), [1, 5].to_vec());
            assert_eq!(factors(12), [1, 2, 3, 4, 6, 12].to_vec());
        }

        #[test]
        #[ignore]
        fn test_is_palindrome() {
            assert!(! is_palindrome([5,2,6].to_vec()));

            assert!(is_palindrome([0].to_vec()));
            assert!(is_palindrome([5,2,4,4,2,5].to_vec()));
        }
    }
}

#[cfg(test)]
mod tests_1to10 {
    use crate::problem001to010::*;

    #[test]
    #[ignore]
    fn test_p001() {
        // Problem 1: Multiples of 3 and 5
        assert_eq!(p001::v1(10), 23);
        assert_eq!(p001::v2(10), 23);
    }

    #[test]
    #[ignore]
    fn test_p002() {
        // Problem 2: Even Fibonacci numbers
        assert_eq!(p002::v1(10), 44);
    }

    #[test]
    #[ignore]
    fn test_p003() {
        // Problem 3: Largest prime factor
        assert_eq!(p003::v1(13195), 29);
    }

    #[test]
    #[ignore]
    fn test_p004() {
        // Problem 4: Largest palindrome product
        assert_eq!(p004::v1(2), 9009);
    }

    #[test]
    #[ignore]
    fn test_p005() {
        // Problem 5: Smallest multiple
        assert_eq!(p005::v1(10), 2520);
        assert_eq!(p005::v1(11), 27720);
    }

    #[test]
    #[ignore]
    fn test_p006() {
        // Problem 6: Sum Square Difference
        assert_eq!(p006::v1(10), 2640);
    }

    #[test]
    // #[ignore]
    fn test_p007() {
        // Problem 7: n-th prime
        assert_eq!(p007::v1(6), 13);
    }

    #[test]
    #[ignore]
    fn test_p008() {
        // Problem 8: Largest product in a series
        assert_eq!(p008::v1(4), 5832);
    }

    #[test]
    #[ignore]
    fn test_p009() {
        // Problem 9: Special Pythagorean triplet
        let abc = p009::v1();
        assert_eq!(abc.0.pow(2) + abc.1.pow(2), abc.2.pow(2));
        assert_eq!(abc.0 + abc.1 + abc.2, 1000);

        let abc = p009::v2();
        assert_eq!(abc.0.pow(2) + abc.1.pow(2), abc.2.pow(2));
        assert_eq!(abc.0 + abc.1 + abc.2, 1000);
    }

    #[test]
    #[ignore]
    fn test_p010() {
        // Problem 10: Summation of primes
        assert_eq!(p010::v1(10), 17);
    }
}

#[cfg(test)]
mod tests_11to20 {
    use crate::problem011to020::*;

    #[test]
    #[ignore]
    fn test_p012() {
        assert_eq!(p012::v1(5), 28);
    }

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
