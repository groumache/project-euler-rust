// Problem 48: Self powers
//   Find the last ten digits of the series, 1^1 + 2^2 + ... + 1000^1000.
// Problem 49: Prime permutations
//   What 12-digit number do you form by concatenating the three terms in this sequence?
// Problem 50: Consecutive prime sum
//   Which prime, below one-million, can be written as the sum of the most consecutive primes?

// What is the largest n-digit pandigital prime below 'max'?
pub mod p041 {
    use crate::useful_func::prime_numbers::primes_range;
    use crate::useful_func::digits::{num_to_digits, no_double};

    pub fn v1(max: u32) -> u32 {
        primes_range(1, max)
            .rev()
            .filter(|x| {
                let mut digits = num_to_digits(*x);
                no_double(&mut digits)
            })
            .next()
            .unwrap()
    }
}

// Using words.txt, a 16K text file containing nearly two-thousand
// common English words, how many are triangle words ?
pub mod p042 {
    use std::fs;
    use crate::useful_func::other_func::word_value;
    use crate::useful_func::triangle_num::is_triangle;

    pub fn v1() -> u32 {
        let filename = "p042_words.txt";
        let contents = fs::read_to_string(filename).expect("Problem with reading the file");
        let words: Vec<&str> = contents.split(',').collect();

        words.iter()
            .filter(|w| is_triangle(word_value(w)))
            .count() as u32
    }
}

// Find the sum of all 0 to 9 pandigital numbers such that the number formed by
// d_2, d_3, d_4 is divisible by 2, the number formed by d_3, d_4, d_5 is
// divisible by 3, the number formed by d_4, d_5, d_6 is divisible by 5, the
// number formed by d_5, d_6, d_7 is divisible by 7, the number formed by
// d_6, d_7, d_8 is divisible by 11, the number formed by d_7, d_8, d_9 is
// divisible by 13 and the number formed by d_8, d_9, d_10 is divisible by 17.
pub mod p043 {
    use crate::useful_func::permutations::permutations;
    use crate::useful_func::digits::digits_to_num;
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
    use crate::useful_func::triangle_num::triangles;
    use crate::useful_func::pentagonal_num::is_pentagon;
    use crate::useful_func::hexagonal_num::is_hexagon;

    pub fn v1() -> u32 {
        triangles()
            .filter(|x| is_pentagon(*x) && is_hexagon(*x))
            .next().unwrap()
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
    fn is_prime(n: u32) -> bool {
        let half = n / 2;
        for i in 2..=half {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    struct Primes {
        n: u32,
        largest: u32,
    }
    impl Iterator for Primes {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            self.n += 1;
            for i in self.largest.. {
                if is_prime(i) {
                    self.largest = i;
                    break;
                }
            }
            Some(self.largest)
        }
    }
    fn primes() -> Primes {
        Primes { n: 0, largest: 1 }
    }
    fn prime_factors(n: u32) -> Vec<u32> {
        let mut n = n;
        let mut p_fact: Vec<u32> = Vec::new();
        for i in primes() {
            if n % i == 0 {
                p_fact.push(i);
                n /= i;
            }
            if n == 1 {
                break;
            }
        }
        p_fact
    }
    fn no_double(v: &mut Vec<u32>) -> bool {
        let length = v.len();
        v.sort();
        v.dedup();
        if length != v.len() {
            return false;
        }
        true
    }
    pub fn v1() -> u32 {
        let mut first: u32 = 0;
        for i in 1.. {
            let mut p_fact: Vec<u32> = Vec::new();
            for j in 0..4 {
                p_fact.append(&mut prime_factors(i + j));
            }
            if no_double(&mut p_fact) {
                first = i;
                break;
            }
        }
        first
    }
}

pub mod p048 {
    pub fn v1() -> u32 {
        let mut ten_digits: u32 = 0;
        let max: u32 = 1000;
        for i in 1..max {
            let base_10: u32 = 10;
            ten_digits += i.pow(i) % base_10.pow(10);
        }
        ten_digits
    }
}

pub mod p049 {
    fn get_digits(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log10() as u32;
        for i in 0..=length {
            let base: u32 = 10;
            let digit: u32 = n / base.pow(i) % 10;
            digits.push(digit);
        }
        digits
    }
    fn get_number(digits: &Vec<u32>) -> u32 {
        let mut num: u32 = 0;
        let base: u32 = 10;
        for (i, d) in digits.iter().enumerate() {
            num += d * base.pow(i as u32);
        }
        num
    }
    fn is_prime(n: u32) -> bool {
        let half = n / 2;
        for i in 2..=half {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    struct Primes {
        p: u32,
        maximum: u32,
        no_max: bool,
    }
    impl Iterator for Primes {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            for i in self.p.. {
                if !self.no_max && i > self.maximum {
                    return None;
                }
                if is_prime(i) {
                    self.p = i;
                    break;
                }
            }
            Some(self.p)
        }
    }
    fn primes(min: u32, max: u32) -> Primes {
        Primes {
            p: min,
            maximum: max,
            no_max: false,
        }
    }
    fn no_double(v: &mut Vec<u32>) -> bool {
        let length = v.len();
        v.sort();
        v.dedup();
        if length != v.len() {
            return false;
        }
        true
    }
    fn is_permutation(v1: &mut Vec<u32>, v2: &mut Vec<u32>) -> bool {
        v1.sort();
        v2.sort();
        if v1 == v2 {
            return true;
        }
        false
    }
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
        for i in primes(min, max) {
            let mut digits1 = get_digits(i);
            if !no_double(&mut digits1) {
                continue;
            }
            p1 = i;
            for j in primes(i, max) {
                let mut digits2 = get_digits(j);
                if !no_double(&mut digits2) && is_permutation(&mut digits1, &mut digits2) {
                    continue;
                }
                p2 = j;
                for k in primes(j, max) {
                    digits2 = get_digits(k);
                    if !no_double(&mut digits2) && is_permutation(&mut digits1, &mut digits2) {
                        continue;
                    }
                    p3 = k;
                }
            }
        }
        let mut digits = get_digits(p1);
        let mut digits2 = get_digits(p2);
        digits.append(&mut digits2);
        let mut digits2 = get_digits(p3);
        digits.append(&mut digits2);
        get_number(&mut digits)
    }
}

pub mod p050 {
    fn is_prime(n: u32) -> bool {
        let half = n / 2;
        for i in 2..=half {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    struct Primes {
        minimum: u32,
        maximum: u32,
        no_max: bool,
    }
    impl Iterator for Primes {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            for i in self.minimum.. {
                if !self.no_max && i > self.maximum {
                    return None;
                }
                if is_prime(i) {
                    self.minimum = i;
                    break;
                }
            }
            Some(self.minimum)
        }
    }
    /*
        impl DoubleEndedIterator for Primes {
            fn next_back(&mut self) -> Option<u32> {
                let max = self.maximum;
                for i in (self.minimum .. self.maximum).rev() {
                    if is_prime(i) {
                        self.maximum = i;
                        break;
                    }
                }
                if max == self.maximum { return None; }
                Some(self.maximum)
            }
        }
    */
    fn primes(min: u32, max: u32) -> Primes {
        Primes {
            minimum: min,
            maximum: max,
            no_max: false,
        }
    }
    fn primes_below(n: u32) -> Vec<u32> {
        let mut primes: Vec<u32> = Vec::new();
        for i in 2..n {
            if is_prime(i) {
                primes.push(i);
            }
        }
        primes
    }
    pub fn v1() -> u32 {
        let mut prime: u32 = 0;
        let mut sum_len: usize = 0;
        let min = 1;
        let max = 1_000_000;
        for p in primes(min, max) {
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
