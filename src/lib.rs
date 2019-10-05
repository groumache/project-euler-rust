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

        #[test]
        #[ignore]
        fn test_fibonacci() {
            let mut fib: Vec<u32> = Vec::new();

            for i in fibonacci() {
                if i > 10 {
                    break;
                }
                fib.push(i);
            }

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
        fn test_frac_digits() {
            assert_eq!(frac_digits(10, 3, 5), [3, 3, 3, 3, 3].to_vec());
            assert_eq!(frac_digits(1, 7, 6), [1, 4, 2, 8, 5, 7].to_vec());
        }

        #[test]
        #[ignore]
        fn test_no_double() {
            assert!(no_double(&mut [5, 7, 8].to_vec()));
            assert!(no_double(&mut [0].to_vec()));
            assert!(! no_double(&mut [5, 5].to_vec()));
            assert!(! no_double(&mut [4, 8, 4].to_vec()));
        }

        #[test]
        #[ignore]
        fn test_get_grid_digits() {
            let grid = "01 02 03 04";
            let width = 2;
            
            let mut num_grid: Vec<Vec<u32>> = Vec::new();
            let row1 = [1, 2].to_vec();
            let row2 = [3, 4].to_vec();
            num_grid.push(row1);
            num_grid.push(row2);

            assert_eq!(get_grid_digits(&grid, width), num_grid);
        }

        #[test]
        #[ignore]
        fn test_get_triangle_digits() {
            let triangle = "01 02 03";

            let mut num_triangle: Vec<Vec<u32>> = Vec::new();
            let row1 = [1].to_vec();
            let row2 = [2, 3].to_vec();
            num_triangle.push(row1);
            num_triangle.push(row2);

            assert_eq!(get_triangle_digits(&triangle), num_triangle);
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
        fn test_is_amicable() {
            assert!(is_amicable(220));
            assert!(is_amicable(63020));

            assert!(! is_amicable(15));
            assert!(! is_amicable(256));
        }

        #[test]
        #[ignore]
        fn test_is_perfect() {
            assert!(is_perfect(28));

            assert!(!is_perfect(14));
        }

        #[test]
        #[ignore]
        fn test_is_deficient() {
            assert!(is_deficient(10));

            assert!(!is_deficient(28));
        }

        #[test]
        #[ignore]
        fn test_is_abundant() {
            assert!(is_abundant(12));

            assert!(!is_abundant(15));
        }

        #[test]
        #[ignore]
        fn test_factors() {
            assert_eq!(factors(5), [1, 5].to_vec());
            assert_eq!(factors(12), [1, 2, 3, 4, 6, 12].to_vec());
        }

        #[test]
        #[ignore]
        fn test_fact() {
            assert_eq!(fact(0), 1);
            assert_eq!(fact(1), 1);
            assert_eq!(fact(5), 120);
        }

        #[test]
        #[ignore]
        fn test_is_palindrome() {
            assert!(! is_palindrome([5,2,6].to_vec()));

            assert!(is_palindrome([0].to_vec()));
            assert!(is_palindrome([5,2,4,4,2,5].to_vec()));
        }

        #[test]
        #[ignore]
        fn test_is_pythagorean_triplet() {
            assert!(is_pythagorean_triplet(3, 4, 5));

            assert!(!is_pythagorean_triplet(6, 7, 8));
        }

        #[test]
        #[ignore]
        fn test_get_substring() {
            let string: String = "0123456789".to_string();
            assert_eq!(get_substring(&string, 3, 5), "34567".to_string());
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
    #[ignore]
    fn test_p007() {
        // Problem 7: n-th prime
        assert_eq!(p007::v1(6), 13);
    }

    #[test]
    #[ignore]
    fn test_p008() {
        // Problem 8: Largest product in a series
        let num_string = String::from("73167176531330624919225119674426574742355349194934")
            + "96983520312774506326239578318016984801869478851843"
            + "85861560789112949495459501737958331952853208805511"
            + "12540698747158523863050715693290963295227443043557"
            + "66896648950445244523161731856403098711121722383113"
            + "62229893423380308135336276614282806444486645238749"
            + "30358907296290491560440772390713810515859307960866"
            + "70172427121883998797908792274921901699720888093776"
            + "65727333001053367881220235421809751254540594752243"
            + "52584907711670556013604839586446706324415722155397"
            + "53697817977846174064955149290862569321978468622482"
            + "83972241375657056057490261407972968652414535100474"
            + "82166370484403199890008895243450658541227588666881"
            + "16427171479924442928230863465674813919123162824586"
            + "17866458359124566529476545682848912883142607690042"
            + "24219022671055626321111109370544217506941658960408"
            + "07198403850962455444362981230987879927244284909188"
            + "84580156166097919133875499200524063689912560717606"
            + "05886116467109405077541002256983155200055935729725"
            + "71636269561882670428252483600823257530420752963450";
        assert_eq!(p008::v1(4, &num_string), 5832);
    }

    #[test]
    #[ignore]
    fn test_p009() {
        // Problem 9: Special Pythagorean triplet
        let abc = p009::v1();
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
    fn test_p011() {
        let grid: String =
            "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08".to_string()
            + " 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00"
            + " 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65"
            + " 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91"
            + " 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80"
            + " 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50"
            + " 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70"
            + " 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21"
            + " 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72"
            + " 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95"
            + " 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92"
            + " 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57"
            + " 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58"
            + " 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40"
            + " 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66"
            + " 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69"
            + " 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36"
            + " 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16"
            + " 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54"
            + " 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";
        let width: usize = 20;
        let n_adjacent: usize = 2;

        assert_eq!(p011::v1(&grid, width, n_adjacent), 9603);
    }

    #[test]
    #[ignore]
    fn test_p012() {
        assert_eq!(p012::v1(5), 28);
    }

    #[test]
    #[ignore]
    fn test_p013() {
        let mut vec_num: Vec<&str> = Vec::new();
        vec_num.push("000000000000000");
        vec_num.push("111110000011111");
        vec_num.push("333330000044444");
        assert_eq!(p013::v1(vec_num, 10), 55555);
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
    fn test_p018() {
        let triangle_1 = "01  02 03";
        let triangle_2 = "01  00 01  79 00 01";

        assert_eq!(p018::v1(triangle_1),  4);
        assert_eq!(p018::v1(triangle_2), 80);
    }

    #[test]
    #[ignore]
    fn test_p020() {
        assert_eq!(p020::v1(10), 27);
    }
}

#[cfg(test)]
mod tests_21to30 {
    use crate::problem021to030::*;

    #[test]
    #[ignore]
    fn test_p021() {
        assert_eq!(p021::v1(100), 0);
        assert_eq!(p021::v1(1000), 2);
    }

    #[test]
    #[ignore]
    fn test_p023() {
        assert_eq!(p023::v1(20), 19);
        assert_eq!(p023::v1(25), 23);
    }

    #[test]
    #[ignore]
    fn test_p024() {
        assert_eq!(p024::v1(0), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec());
        assert_eq!(p024::v1(1), [0, 1, 2, 3, 4, 5, 6, 7, 9, 8].to_vec());
        assert_eq!(p024::v1(2), [0, 1, 2, 3, 4, 5, 6, 8, 7, 9].to_vec());
    }

    #[test]
    #[ignore]
    fn test_p025() {
        assert_eq!(p025::v1(2), 13);
        assert_eq!(p025::v1(3), 144);
    }

    #[test]
    #[ignore]
    fn test_p026() {
        assert_eq!(p026::v1(10), 7);
    }

    #[test]
    #[ignore]
    fn test_p027() {
        assert_eq!(p027::v1(50), 41);
    }

    #[test]
    fn test_p028() {
        assert_eq!(p028::v1(5), 101);
    }

    #[test]
    #[ignore]
    fn test_p029() {
        assert_eq!(p029::v1(5), 15);
    }
}

