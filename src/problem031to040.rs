// How many different ways can £2 be made using any number of coins ?
pub mod p031 {
    use crate::useful_func::coins::Coins;

    // We just try everything in an orderly manner.
    pub fn v1(purse: Coins) -> u32 {
        let mut counter: u32 = 1;
        let mut curr: Coins = purse;
        let pence = curr.p1
            + curr.p2 * 2
            + curr.p5 * 5
            + curr.p10 * 10
            + curr.p20 * 20
            + curr.p50 * 50
            + curr.pound1 * 100
            + curr.pound2 * 100;

        let terminal = Coins {
            pound2: 0,
            pound1: 0,
            p50: 0,
            p20: 0,
            p10: 0,
            p5: 0,
            p2: 0,
            p1: pence,
        };

        while curr != terminal {
            counter += 1;
            if curr.p2 != 0 {
                curr.p1 += 2;
                curr.p2 -= 1;
                continue;
            } else if curr.p5 != 0 {
                curr.p1 += 1;
                curr.p2 += 2;
                curr.p5 -= 1;
                continue;
            } else if curr.p10 != 0 {
                curr.p5 += 2;
                curr.p10 -= 1;
                continue;
            } else if curr.p20 != 0 {
                curr.p10 += 2;
                curr.p20 -= 1;
                continue;
            } else if curr.p50 != 0 {
                curr.p10 += 1;
                curr.p20 += 2;
                curr.p50 -= 1;
                continue;
            } else if curr.pound1 != 0 {
                curr.p50 += 2;
                curr.pound1 -= 1;
                continue;
            } else if curr.pound2 != 0 {
                curr.pound1 += 2;
                curr.pound2 -= 1;
                continue;
            }
        }

        counter
    }
}

// Find the sum of all products whose multiplicand/multiplier/product
// identity can be written as a 1 through 9 pandigital.
pub mod p032 {
    use crate::useful_func::digits::{no_double, num_to_digits};
    use crate::useful_func::other_func::factors;

    // Such a number can only have 4 digits
    pub fn v1() -> u32 {
        let mut sum: u32 = 0;
        for num in 1234..=9876 {
            let mut digits: Vec<u32> = num_to_digits(num);
            if !no_double(&mut digits) {
                continue;
            }

            let divisors: Vec<u32> = factors(num);
            for div in divisors {
                let quotient = num / div;
                let mut digits_div = num_to_digits(div);
                let mut digits_quo = num_to_digits(quotient);
                digits.append(&mut digits_div);
                digits.append(&mut digits_quo);

                if no_double(&mut digits) {
                    sum += num;
                }
            }
        }

        sum
    }
}

// Problem 33: Digit cancelling fractions
pub mod p033 {
    use crate::useful_func::digits::num_to_digits;
    use crate::useful_func::other_func::get_gcd;

    pub fn no_shared_digits(v1: &Vec<u32>, v2: &Vec<u32>) -> bool {
        v1.iter().any(|x| v2.contains(x))
    }

    pub fn v1() -> u32 {
        let mut num: Vec<u32> = Vec::new();
        let mut den: Vec<u32> = Vec::new();

        for numerator in (11..99).filter(|n| n % 10 != 0) {
            for denominator in (11..99).filter(|n| n % 10 != 0) {
                let digits_num = num_to_digits(numerator);
                let digits_den = num_to_digits(denominator);
                if no_shared_digits(&digits_den, &digits_num) {
                    continue;
                }

                let d_num: u32 = *digits_num.iter().find(|d| !digits_den.contains(d)).unwrap();
                let d_den: u32 = *digits_den.iter().find(|d| !digits_num.contains(d)).unwrap();

                let frac1 = numerator / denominator;
                let frac2 = d_num / d_den;

                if frac1 == frac2 as u32 {
                    num.push(d_num);
                    den.push(d_den);
                }
            }
        }

        let prod_num: u32 = num.iter().product();
        let prod_den: u32 = den.iter().product();

        let gcd: u32 = get_gcd(prod_num, prod_den);
        prod_den / gcd
    }
}

// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
pub mod p034 {
    use crate::useful_func::digits::num_to_digits;
    use crate::useful_func::other_func::fact;

    // max 7 digits as (9!) * 8 has 7 digits
    pub fn v1() -> u32 {
        let max = 8 * fact(9);

        (11..max)
            .filter(|x| *x == num_to_digits(*x).iter().map(|d| fact(*d)).sum())
            .sum()
    }
}

// How many circular primes are there below 'n'?
pub mod p035 {
    use crate::useful_func::other_func::is_circular_prime;
    use crate::useful_func::prime_numbers::primes_range;

    pub fn v1(n: u32) -> u32 {
        primes_range(0, n).filter(|x| is_circular_prime(*x)).count() as u32
    }
}

// Find the sum of all palindromic numbers, in base 10 and 2, below 'n'.
pub mod p036 {
    use crate::useful_func::digits::{num_to_binary_digits, num_to_digits};
    use crate::useful_func::other_func::is_palindrome;

    pub fn v1(n: u32) -> u32 {
        (1..n)
            .filter(|x| is_palindrome(num_to_digits(*x)) && is_palindrome(num_to_binary_digits(*x)))
            .count() as u32
    }
}

// Find the sum of the only 11 primes that are both truncatable from left to
// right and right to left.
pub mod p037 {
    use crate::useful_func::prime_numbers::{
        is_left_truncable_prime, is_right_truncable_prime, primes,
    };

    pub fn v1() -> u32 {
        primes()
            .filter(|x| is_right_truncable_prime(*x) && is_left_truncable_prime(*x))
            .take(11)
            .sum()
    }
}

// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the
// concatenated product of an integer with (1, 2, ... , n) where n > 1 ?
pub mod p038 {
    use crate::useful_func::digits::{no_double, num_to_digits, digits_to_num};

    // max 4 digits
    pub fn v1() -> u32 {
        let mut max_pandigital = 0;
        let max = 100_000;

        for i in 2..max {
            let mut digits: Vec<u32> = Vec::new();

            for j in 1.. {
                let num: u32 = i * j;
                digits.append(&mut num_to_digits(num));
                if digits.len() >= 9 {
                    break;
                }
            }

            // is_1_to_9_pandigital(digits)
            if digits.len() != 9 && !no_double(&mut digits) && !digits.contains(&0) {
                continue;
            }

            let new_pandigital = digits_to_num(&mut digits);
            max_pandigital = std::cmp::max(new_pandigital, max_pandigital);
        }

        max_pandigital
    }
}

// p = perimeter of triangle with sides {a,b,c}, where a, b, c are integers.
// For which value of p ≤ 1000, is the number of solutions maximised ?
pub mod p039 {
    pub fn v1() -> u32 {
        let mut p_max = 0;
        let mut nb_solutions = 0;

        for p in 3..=1000 {
            let mut counter = 0;

            for a in 1..p {
                for b in a..p {
                    for c in b..p {
                        if a + b + c == p {
                            counter += 1;
                        }
                    }
                    // counter += (b..p).filter(|c| a + b + c == p).count();
                }
            }

            if counter > nb_solutions {
                nb_solutions = counter;
                p_max = p;
            }
        }

        p_max
    }
}

// If d_n represents the nth digit of the fractional part of 0.1 2 3 ... 9 10 11 ...,
// find the value of the following expression:
// d_1 × d_10 × d_100 × d_1000 × d_10000 × d_100000 × d_1000000
pub mod p040 {
    use crate::useful_func::digits::num_to_digits;

    // no need to actually compute the fraction
    pub fn v1() -> u32 {
        let mut expression: u32 = 1;
        let d_x: [u32; 7] = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];
        let mut digit_num: u32 = 1;

        for i in 1..1_000_000 {
            let digits = num_to_digits(i);

            for (j, digit) in digits.iter().enumerate() {
                let x = digit_num + j as u32;

                // could make it faster by checking if 'x == 10^y' with y being
                // incremented until 'y == 7', then break;
                if d_x.contains(&x) {
                    expression *= digit;
                }
            }

            digit_num += digits.len() as u32;
        }

        expression
    }
} // NOT SIMPLIFIED
