// Problem 31: Coin sums
//   How many different ways can £2 be made using any number of coins ?
// Problem 32: Pandigital products
//   Find the sum of all products whose multiplicand/multiplier/product
//   identity can be written as a 1 through 9 pandigital.
// Problem 33: Digit cancelling fractions
// Problem 34: Digit factorials
//   Find the sum of all numbers which are equal to the sum of the factorial of their digits.
// Problem 35: Circular primes
//   How many circular primes are there below one million?
// Problem 36: Double-base palindromes
//   Find the sum of all numbers, less than one million, which are palindromic in
//   base 10 and base 2.
// Problem 37: Truncatable primes
//   Find the sum of the only eleven primes that are both truncatable from left to
//   right and right to left.
// Problem 38: Pandigital multiples
//   What is the largest 1 to 9 pandigital 9-digit number that can be formed as the
//   concatenated product of an integer with (1,2, ... , n) where n > 1?
// Problem 39: Integer right triangles
//   For which value of p ≤ 1000, is the number of solutions maximised?
// Problem 40: Champernowne's constant
//   If d_n represents the nth digit of the fractional part of 0.1 2 3 ... 9 10 11 ...,
//   find the value of the following expression:
//   d_1 × d_10 × d_100 × d_1000 × d_10000 × d_100000 × d_1000000
pub mod p031 {
    struct Coins {
        pound2: u8,
        pound1: u8,
        p50: u8,
        p20: u8,
        p10: u8,
        p5: u8,
        p2: u8,
        p1: u8,
    }
    impl PartialEq for Coins {
        fn eq(&self, other: &Self) -> bool {
            self.pound2 == other.pound2 && self.pound1 == other.pound1
                && self.p50 == other.p50 && self.p20 == other.p20
                && self.p10 == other.p10 && self.p5 == other.p5
                && self.p2 == other.p2 && self.p1 == other.p1
        }
    }
    // We just try everything in an orderly manner.
    pub fn v1() -> u32 {
        let mut counter: u32= 1;
        let mut curr: Coins = Coins { pound2: 1, pound1: 0, p50: 0, p20: 0, p10: 0, p5: 0, p2: 0, p1: 0 };
        let terminal = Coins { pound2: 0, pound1: 0, p50: 0, p20: 0, p10: 0, p5: 0, p2: 0, p1: 200 };
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
            } else if curr.p50 !=0 {
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

pub mod p032 {
    fn get_digits(n: u32) -> Vec<u8> {
        let mut digits: Vec<u8> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
            let base: u32 = 10;
            let digit: u8 = (n / base.pow(i) % 10) as u8;
            digits.push(digit);
        }
        digits
    }
    fn get_divisors(n: u32) -> Vec<u32> {
        let mut divisors: Vec<u32> = Vec::new();
        for i in 1..n+1 {
            if n % i == 0 { divisors.push(i); }
        }
        divisors
    }
    fn no_double(digits: &mut Vec<u8>) -> bool {
        digits.sort();
        let length = digits.len();
        digits.dedup();
        length == digits.len()
    }
    // Such a number can only have 4 digits
    pub fn v1() -> u32 {
        let mut sum: u32 = 0;
        for num in 1234..9876+1 {
            //                                                              let mut digits = get_digits(num).sort();
            //                                                              if digits.len() != digits.dedup().len() { continue; }
            let mut digits: Vec<u8> = get_digits(num);
            if !no_double(&mut digits) { continue; }

            let divisors: Vec<u32> = get_divisors(num);
            for div in divisors {
                let quotient = num / div;
                let mut digits_div = get_digits(div);
                let mut digits_quo = get_digits(quotient);
                digits.append(&mut digits_div);
                digits.append(&mut digits_quo);

                if no_double(&mut digits) { sum += num; }
            }
        }
        sum
    }
}

pub mod p033 {
    fn get_digits(n: u32) -> Vec<u8> {
        let mut digits: Vec<u8> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
            let base: u32 = 10;
            let digit: u8 = (n / base.pow(i) % 10) as u8;
            digits.push(digit);
        }
        digits
    }
    fn no_double(digits1: &mut Vec<u8>, mut digits2: &mut Vec<u8>) -> bool {
        let mut digits: Vec<u8> = digits1.clone();
        digits.append(&mut digits2);
        digits.sort();
        let length = digits.len();
        digits.dedup();
        length == digits.len()
    }
    fn get_divisors(n: u32) -> Vec<u32> {
        let mut divisors: Vec<u32> = Vec::new();
        for i in 1..n+1 {
            if n % i == 0 { divisors.push(i); }
        }
        divisors
    }
    fn get_gcd(n1: u32, n2: u32) -> u32 {
        let mut gcd: u32 = 1;
        let div1 = get_divisors(n1);
        let div2 = get_divisors(n2);
        for d in div1.iter() {
            if div2.contains(d) { gcd *= d; }
        }
        gcd
    }
    pub fn v1() -> u32 {
        let mut num: Vec<u8> = Vec::new();
        let mut den: Vec<u8> = Vec::new();
        for numerator in (11..99).filter(|n| n % 10 != 0) {
            for denominator in (11..99).filter(|n| n % 10 != 0) {
                let mut digits_num = get_digits(numerator);
                let mut digits_den = get_digits(denominator);
                if no_double(&mut digits_den, &mut digits_num) { continue; }
                let mut common_digit: u8 = 0;
                let mut other_d_num: u8 = 0;
                let mut other_d_den: u8 = 0;
                for digit in digits_num.iter() {
                    if digits_den.contains(digit) {
                        common_digit = *digit;
                    } else {
                        other_d_num = *digit;
                    }
                }
                for digit in digits_den.iter() {
                    if *digit != common_digit {
                        other_d_den = *digit;
                    }
                }
                let frac1 = numerator / denominator;
                let frac2 = other_d_num / other_d_den;
                if frac1 == frac2 as u32 {
                    num.push(other_d_num);
                    den.push(other_d_den);
                }
            }
        }
        let mut prod_num: u32 = 1;
        let mut prod_den: u32 = 1;
        for i in num.iter() { prod_num *= *i as u32; }
        for i in den.iter() { prod_den *= *i as u32; }
        let gcd: u32 = get_gcd(prod_num, prod_den);
        prod_den / gcd
    }
}

pub mod p034 {
    fn get_digits(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
            let base: u32 = 10;
            let digit: u32 = n / base.pow(i) % 10;
            digits.push(digit);
        }
        digits
    }
    fn fact(n: u32) -> u32 {
        if n == 0 { return 1; }
        (1..n+1).product()
    }
    // max 7 digits as (9!) * 8 has 7 digits
    pub fn v1() -> u32 {
        let mut sum: u32 = 0;
        let max = 7 * fact(9);
        for i in 11..max {
            let digits = get_digits(i);
            let sum_fact_digits: u32 = digits.iter().map(|d| fact(*d)).sum();
            if sum_fact_digits == i { sum += i; }
        }
        sum
    }
}

pub mod p035 {
    fn get_digits(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
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
        let half = n / 2 + 1;
        for i in 2..half {
            if n % i == 0 { return false; }
        }
        true
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
        let mut counter = 0;
        let max = 1_000_000;
        let primes = primes_below(max);
        let mut primes_checked: Vec<u32> = Vec::new();
        for p in &primes {
            let mut digits = get_digits(*p);
            let mut circular_prime: bool = true;
            primes_checked.push(*p);
            // check if it's a circular prime
            for _ in 0..digits.len() {
                let d = digits.remove(0);
                digits.push(d);
                let num = get_number(&digits);
                primes_checked.push(num);
                if primes_checked.contains(&num) || !&primes.contains(&num) {
                    circular_prime = false;
                    break;
                }
            }
            if circular_prime { counter += 1; }
        }
        counter
    }
}

pub mod p036 {
    fn get_digits_10(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
            let base: u32 = 10;
            let digit: u32 = n / base.pow(i) % 10;
            digits.push(digit);
        }
        digits
    }
    fn get_digits_2(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log2() as u32 + 1;
        for i in 0..length {
            let base: u32 = 2;
            let digit: u32 = n / base.pow(i) % 10;
            digits.push(digit);
        }
        digits
    }
    fn is_palyndromic(vec: Vec<u32>) -> bool {
        for i in 0..vec.len() {
            let last = vec.len() - 1;
            if vec[i] != vec[last-i] { return false; }
        }
        true
    }
    pub fn v1() -> u32 {
        let mut sum: u32 = 0;
        let max = 1_000_000;
        for i in 1..max {
            let digits_10 = get_digits_10(i);
            let digits_2 = get_digits_2(i);
            if is_palyndromic(digits_10) && is_palyndromic(digits_2) {
                sum += i;
            }
        }
        sum
    }
}

pub mod p037 {
    fn is_prime(n: u32) -> bool {
        let half = n / 2 + 1;
        for i in 2..half {
            if n % i == 0 { return false; }
        }
        true
    }
    fn get_digits(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
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
    pub fn v1() -> u32 {
        let n_truncatable = 11;
        let mut truncatables: Vec<u32> = Vec::new();
        for i in 11.. {
            let mut num = i;
            let mut trunc: bool = true;
            while (num as f64).log10() >= 1.0 {
                let mut digits: Vec<u32> = get_digits(i);
                digits.remove(0);
                num = get_number(&digits);
                if !is_prime(num) {
                    trunc = false;
                    break;
                }
            }
            if trunc { truncatables.push(i); }
            if truncatables.len() == n_truncatable { break; }
        }
        //
        truncatables.iter().sum()
    }
}

pub mod p038 {
    fn get_digits(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
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
    fn no_double(digits: &mut Vec<u32>) -> bool {
        digits.sort();
        let length = digits.len();
        digits.dedup();
        length == digits.len()
    }
    // max 4 number
    pub fn v1() -> u32 {
        let mut max_pandigital = 0;
        let max = 100_000;
        for i in 2..max {
            let mut digits: Vec<u32> = Vec::new();
            // digits = get_pandigital()
            for j in 1.. {
                let num: u32 = i * j;
                digits.append(&mut get_digits(num));
                if digits.len() >= 9 { break; }
            }
            // digit.is_pandigital()
            if digits.len() != 9 || !no_double(&mut digits) { continue; }
            let new_pandigital = get_number(&mut digits);
            if new_pandigital > max_pandigital {
                max_pandigital = new_pandigital;
            }
        }
        max_pandigital
    }
}

pub mod p039 {
    pub fn v1() -> u32 {
        let max_p = 1001;
        let mut p_solutions: Vec<u32> = vec!(0; max_p);
        for a in 1..max_p {
            for b in 1..max_p {
                let a = a as f64;
                let b = b as f64;
                let c = (a.powi(2) + b.powi(2)).sqrt();
                let p = a + b + c;
                if c - c.floor() > 0.0 || p >= max_p as f64 {
                    continue;
                }
                p_solutions[p as usize] += 1;
            }
        }
        let mut max_p = 0;
        let mut max_n: u32 = 0;
        for (p, n) in p_solutions.iter().enumerate() {
            if *n > max_n {
                max_p = p;
                max_n = *n;
            }
        }
        max_p as u32
    }
}

pub mod p040 {
    fn get_digits(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
            let base: u32 = 10;
            let digit: u32 = n / base.pow(i) % 10;
            digits.push(digit);
        }
        digits
    }
    // no need to actually compute the fraction
    pub fn v1() -> u32 {
        let mut expression: u32 = 1;
        let d_x: [u32; 7] = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];
        let mut digit_num: u32 = 1;
        for i in 1..1_000_000 {
            let digits = get_digits(i);
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
}


