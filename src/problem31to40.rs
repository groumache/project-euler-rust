// Problem 31: Coin sums
//   How many different ways can £2 be made using any number of coins ?
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

// Problem 32: Pandigital products
//   Find the sum of all products whose multiplicand/multiplier/product
//   identity can be written as a 1 through 9 pandigital.
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

// Problem 33: Digit cancelling fractions
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

