// What is the largest n-digit pandigital prime below 'max'?
pub mod p041 {
    use crate::useful_func::digits::{no_double, num_to_digits};
    use crate::useful_func::prime_numbers::primes_range;

    pub fn v1(max: u32) -> u32 {
        primes_range(1, max)
            .rev()
            .find(|x| {
                let mut digits = num_to_digits(*x);
                no_double(&mut digits)
            })
            .unwrap()
    }
}

// Using words.txt, a 16K text file containing nearly two-thousand
// common English words, how many are triangle words ?
pub mod p042 {
    use crate::useful_func::other_func::word_value;
    use crate::useful_func::triangle_num::is_triangle;
    use std::fs;

    pub fn v1() -> u32 {
        let filename = "p042_words.txt";
        let contents = fs::read_to_string(filename).expect("Problem with reading the file");
        let words: Vec<&str> = contents.split(',').collect();

        words.iter().filter(|w| is_triangle(word_value(w))).count() as u32
    }
}

// Find the sum of all 0 to 9 pandigital numbers such that the number formed by
// d_2, d_3, d_4 is divisible by 2, the number formed by d_3, d_4, d_5 is
// divisible by 3, the number formed by d_4, d_5, d_6 is divisible by 5, the
// number formed by d_5, d_6, d_7 is divisible by 7, the number formed by
// d_6, d_7, d_8 is divisible by 11, the number formed by d_7, d_8, d_9 is
// divisible by 13 and the number formed by d_8, d_9, d_10 is divisible by 17.
pub mod p043 {
    use crate::useful_func::digits::digits_to_num;
    use crate::useful_func::permutations::permutations;
    use crate::useful_func::prime_numbers::nth_prime;

    pub fn v1() -> u32 {
        let mut sum_num = 0;
        let digits: Vec<u32> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();

        for p in permutations(digits) {
            // test the property
            let mut property = true;
            for (i, w) in p.windows(3).enumerate().skip(1) {
                property &= (digits_to_num(w) % nth_prime(i + 1)) == 0;
            }

            if property {
                sum_num += digits_to_num(&p);
            }
        }

        sum_num
    }
}

// Find the pair of pentagonal numbers, P_j and P_k, for which their sum and
// difference are pentagonal and D = |P_k − P_j| is minimised; what is the value of D ?
pub mod p044 {
    use crate::useful_func::pentagonal_num::{is_pentagon, pentagons};

    pub fn v1() -> (u32, u32) {
        let mut pentagonal_num: (u32, u32) = (0, 0);
        for i in pentagons() {
            for j in pentagons().take_while(|&j| j < i) {
                if is_pentagon(i - j) && is_pentagon(i + j) {
                    pentagonal_num = (i, j);
                }
            }

            if pentagonal_num != (0, 0) {
                break;
            }
        }

        pentagonal_num
    }
}

// Find the next triangle number that is also pentagonal and hexagonal.
pub mod p045 {
    use crate::useful_func::hexagonal_num::is_hexagon;
    use crate::useful_func::pentagonal_num::is_pentagon;
    use crate::useful_func::triangle_num::triangles;

    pub fn v1() -> u32 {
        triangles()
            .find(|x| is_pentagon(*x) && is_hexagon(*x))
            .unwrap()
    }
}

// What is the smallest odd composite that cannot be written as the sum
// of a prime and twice a square ?
pub mod p046 {
    use crate::useful_func::prime_numbers::is_prime;

    pub fn v1() -> u32 {
        let mut smallest_odd_comp = 0;

        for i in (9..).step_by(2).filter(|&i| !is_prime(i)) {
            // test the property
            let mut property = false;
            for j in (1_u32..).map(|j| 2 * j.pow(2)).take_while(|&j| j < i) {
                if is_prime(i - j) {
                    property = true;
                    break;
                }
            }

            if !property {
                smallest_odd_comp = i;
                break;
            }
        }

        smallest_odd_comp
    }
}

// Find the first four consecutive integers to have four distinct prime
// factors each. What is the first of these numbers ?
pub mod p047 {
    use crate::useful_func::prime_numbers::prime_factors;

    pub fn v1() -> u32 {
        let mut first_num: u32 = 0;

        for i in 1.. {
            let mut p_fact: Vec<u32> = Vec::new();
            for j in 0..4 {
                p_fact.append(&mut prime_factors(i + j));
            }

            p_fact.sort();
            p_fact.dedup();

            if p_fact.len() == 16 {
                first_num = i;
                break;
            }
        }

        first_num
    }
}

// Find the last ten digits of the series, 1^1 + 2^2 + ... + n^n.
pub mod p048 {
    pub fn v1(n: u64) -> u64 {
        (1..=n).map(|x| x.pow(x as u32) % 10_u64.pow(10)).sum()
    }
}

// What 12-digit number do you form by concatenating the three terms in this sequence ?
pub mod p049 {
    use crate::useful_func::digits::{digits_to_num, no_double, num_to_digits};
    use crate::useful_func::permutations::is_permutation;
    use crate::useful_func::prime_numbers::primes_range;

    pub fn v1() -> u32 {
        // lots of code duplication...
        // could be better with:
        // (0.) initialize: p1/minimum = 1000, maximum = 10_000
        // (1.) p1 = find_next_prime_with_no_double_digits(min: p1, max: maximum)
        // (2.) p2 = find_next_prime_is_permutation(min/perm: p1, max: maximum)
        // (3.) p3 = find_next_prime_is_permutation(min/perm: p2, max: maximum)
        // (4.) if failure with steps (2.) or (3.), go back to step (1.)
        let mut p1: u32 = 0;
        let mut p2: u32 = 0;
        let mut p3: u32 = 0;
        let max: u32 = 10_000;
        let min: u32 = 1000;
        for i in primes_range(min, max) {
            let mut digits1 = num_to_digits(i);
            if !no_double(&mut digits1) {
                continue;
            }
            p1 = i;
            for j in primes_range(i, max) {
                let mut digits2 = num_to_digits(j);
                if !no_double(&mut digits2) && is_permutation(&digits1, &digits2) {
                    continue;
                }
                p2 = j;
                for k in primes_range(j, max) {
                    digits2 = num_to_digits(k);
                    if !no_double(&mut digits2) && is_permutation(&digits1, &digits2) {
                        continue;
                    }
                    p3 = k;
                }
            }
        }
        let mut digits = num_to_digits(p1);
        let mut digits2 = num_to_digits(p2);
        digits.append(&mut digits2);
        let mut digits2 = num_to_digits(p3);
        digits.append(&mut digits2);
        digits_to_num(&digits)
    }
}

// Which prime, below 'n', can be written as the sum of the most consecutive primes ?
pub mod p050 {
    use crate::useful_func::prime_numbers::{primes, primes_below};

    // max number of consecutive primes that could sum up to 'n'
    fn max_win_size(n: u32) -> u32 {
        let mut sum = 0;
        let mut max_num = 0;

        for (i, p) in primes().enumerate() {
            sum += p;

            if sum >= n {
                max_num = i + 1;
                break;
            }
        }

        max_num as u32
    }

    pub fn v1(n: u32) -> u32 {
        let mut prime: u32 = 0;
        let mut sum_len: u32 = 0;

        for p in primes().take_while(|&x| x < n) {
            // find the largest sum of primes that p can be written into
            'outer: for w_size in (2..=max_win_size(p)).rev() {
                for w in primes_below(n).windows(w_size as usize) {
                    let mut w_sum = 0;
                    for i in w {
                        w_sum += i;
                    }

                    if w_sum == p && w_size > sum_len {
                        prime = p;
                        sum_len = w_size;
                        break 'outer;
                    } else if w_sum >= p {
                        break 'outer;
                    }
                }
            }
        }

        prime
    }

    pub fn v2(n: u32) -> u32 {
        let mut prime: u32 = 0;
        let mut sum_len: usize = 0;

        for p in primes().take_while(|&x| x < n) {
            let p_below: Vec<u32> = primes_below(p + 1);
            let mut sum: u32 = 0;
            let mut start: usize = 0;
            let mut stop: usize = 0;

            loop {
                if sum == p {
                    let length = stop - start + 1;
                    if length > sum_len {
                        prime = p;
                        sum_len = length;
                    }
                    break;
                } else if sum > p {
                    start += 1;
                    stop = start;
                    sum = 0;
                } else {
                    // sum < p
                    sum += p_below[stop];
                }
            }
        }

        prime
    }
}
