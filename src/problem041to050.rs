// Problem 41: Pandigital prime
//   What is the largest n-digit pandigital prime that exists?
// Problem 42: Coded triangle numbers
//   Using words.txt, a 16K text file containing nearly two-thousand
//   common English words, how many are triangle words ?
// Problem 43: Sub-string divisibility
//   Find the sum of all 0 to 9 pandigital numbers with this property.
// Problem 44: Pentagon numbers
//   Find the pair of pentagonal numbers, Pj and Pk, for which their sum
//   and difference are pentagonal and D = |Pk − Pj| is minimised; what
//   is the value of D?
// Problem 45: Triangular, pentagonal, and hexagonal
//   Find the next triangle number that is also pentagonal and hexagonal.
// Problem 46: Goldbach's other conjecture
//   What is the smallest odd composite that cannot be written as the sum
//   of a prime and twice a square?
// Problem 47: Distinct primes factors
//   Find the first four consecutive integers to have four distinct prime
//   factors each. What is the first of these numbers?
// Problem 48: Self powers
//   Find the last ten digits of the series, 1^1 + 2^2 + ... + 1000^1000.
// Problem 49: Prime permutations
//   What 12-digit number do you form by concatenating the three terms in this sequence?
// Problem 50: Consecutive prime sum
//   Which prime, below one-million, can be written as the sum of the most consecutive primes?

pub mod p041 {
    fn is_prime(n: u32) -> bool {
        let half = n / 2 + 1;
        for i in 2..half {
            if n % i == 0 {
                return false;
            }
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
    fn no_double(digits: &mut Vec<u32>) -> bool {
        digits.sort();
        let length = digits.len();
        digits.dedup();
        length == digits.len()
    }
    pub fn v1() -> u32 {
        let mut largest_pandigit: u32 = 0;
        let max = 987654321 + 1;
        for i in (1..max).rev() {
            let i = i as u32;
            let mut digits = get_digits(i);
            if is_prime(i) && no_double(&mut digits) {
                largest_pandigit = i;
                break;
            }
        }
        largest_pandigit
    }
}

pub mod p042 {
    use std::fs;
    fn letter_value(l: char) -> u32 {
        match l {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            _ => panic!(),
        }
    }
    fn is_triangle_num(n: u32) -> bool {
        for i in (1..).map(|n| n * (n + 1) / 2) {
            if n == i {
                return true;
            } else if n > i {
                break;
            }
        }
        false
    }
    pub fn v1() -> u32 {
        let mut counter: u32 = 0;
        let filename = "p042_words.txt";
        let contents = fs::read_to_string(filename).expect("Problem with reading the file");
        let words: Vec<&str> = contents.split(',').collect();
        for w in words {
            let w = w.to_string();
            let mut w_value: u32 = 0; // w_value = w.iter().map(|l| letter_value(l)).collect()  ===>  LOL -> that would be cool but it's a bit extreme
            for l in w.chars().filter(|c| *c != '"') {
                w_value += letter_value(l);
            }
            if is_triangle_num(w_value) {
                counter += 1;
            }
        }
        counter
    }
}

pub mod p043 {
    fn get_number(digits: &Vec<u32>) -> u32 {
        let mut num: u32 = 0;
        let base: u32 = 10;
        for (i, d) in digits.iter().enumerate() {
            num += d * base.pow(i as u32);
        }
        num
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
    fn fact(n: u32) -> u32 {
        if n == 0 {
            return 1;
        }
        (1..n + 1).product()
    }
    fn get_permutation<T>(items: Vec<T>, perm: u32) -> Option<Vec<T>> {
        let mut perm = perm;
        let mut items = items;
        // check if permutation is possible
        let n_element = items.len() as u32;
        let n_possible_perm = fact(n_element);
        if perm > n_possible_perm {
            return None;
        }
        // find which element we have to shift
        let mut permutations: Vec<u32> = Vec::new();
        for i in (1..n_element + 1).rev() {
            let factorial: u32 = fact(i + 1);
            permutations.push(perm / factorial);
            perm = perm % factorial;
        }
        // compute the permutations
        let mut final_perm: Vec<T> = Vec::new();
        for i in permutations.iter() {
            final_perm.push(items.remove(*i as usize));
        }
        Some(final_perm)
    }
    struct Pandigital10 {
        perm: u32,
        max: u32,
    }
    impl Iterator for Pandigital10 {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            self.perm += 1;
            if self.perm == self.max {
                return None;
            }
            let mut items: Vec<u32> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
            items = get_permutation(items, self.perm).unwrap();
            Some(get_number(&items))
        }
    }
    fn pandigital() -> Pandigital10 {
        Pandigital10 {
            perm: 0,
            max: fact(10),
        }
    }
    pub fn v1() -> u32 {
        let mut sum_pandigital: u32 = 0;
        for i in pandigital() {
            // this could be much more simple
            let mut digits: Vec<u32> = get_digits(i);
            digits.reverse();
            let mut d234: Vec<u32> = Vec::new();
            d234.push(digits[1]);
            d234.push(digits[2]);
            d234.push(digits[3]);
            let mut d345: Vec<u32> = Vec::new();
            d345.push(digits[2]);
            d345.push(digits[3]);
            d345.push(digits[4]);
            let mut d456: Vec<u32> = Vec::new();
            d456.push(digits[3]);
            d456.push(digits[4]);
            d456.push(digits[5]);
            let mut d567: Vec<u32> = Vec::new();
            d567.push(digits[4]);
            d567.push(digits[5]);
            d567.push(digits[6]);
            let mut d678: Vec<u32> = Vec::new();
            d678.push(digits[5]);
            d678.push(digits[6]);
            d678.push(digits[7]);
            let mut d789: Vec<u32> = Vec::new();
            d789.push(digits[6]);
            d789.push(digits[7]);
            d789.push(digits[8]);
            let mut d8910: Vec<u32> = Vec::new();
            d8910.push(digits[7]);
            d8910.push(digits[8]);
            if digits.len() > 9 {
                d8910.push(digits[9]);
            }

            if get_number(&d234) % 2 == 0
                && get_number(&d345) % 3 == 0
                && get_number(&d456) % 5 == 0
                && get_number(&d567) % 7 == 0
                && get_number(&d678) % 11 == 0
                && get_number(&d789) % 13 == 0
                && get_number(&d8910) % 17 == 0
            {
                sum_pandigital += i;
            }
        }
        sum_pandigital
    }
}

pub mod p044 {
    struct PentagonNum {
        n: u32,
    }
    impl Iterator for PentagonNum {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let n = self.n;
            self.n += 1;
            Some(n * (3 * n - 1) / 2)
        }
    }
    fn pentagon() -> PentagonNum {
        PentagonNum { n: 1 }
    }
    // could check with a 'formula' [O(1)] instead of [O(n)]
    fn is_pentagon(n: u32) -> bool {
        for i in pentagon() {
            if n == i {
                return true;
            } else if n > i {
                break;
            }
        }
        false
    }
    pub fn v1() -> (u32, u32) {
        let mut pentagonal_num: (u32, u32) = (0, 0);
        for i in pentagon() {
            for j in pentagon() {
                if j >= i {
                    break;
                }
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

pub mod p045 {
    struct TriangleNum {
        n: u32,
    }
    impl Iterator for TriangleNum {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let n = self.n;
            self.n += 1;
            Some(n * (n + 1) / 2)
        }
    }
    fn triangle() -> TriangleNum {
        TriangleNum { n: 1 }
    }
    struct PentagonNum {
        n: u32,
    }
    impl Iterator for PentagonNum {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let n = self.n;
            self.n += 1;
            Some(n * (3 * n - 1) / 2)
        }
    }
    fn pentagon() -> PentagonNum {
        PentagonNum { n: 1 }
    }
    // could check with a 'formula' [O(1)] instead of [O(n)]
    fn is_pentagon(n: u32) -> bool {
        for i in pentagon() {
            if n == i {
                return true;
            } else if n > i {
                break;
            }
        }
        false
    }
    struct HexagonalNum {
        n: u32,
    }
    impl Iterator for HexagonalNum {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let n = self.n;
            self.n += 1;
            Some(n * (2 * n - 1))
        }
    }
    fn hexagon() -> HexagonalNum {
        HexagonalNum { n: 1 }
    }
    // could check with a 'formula' [O(1)] instead of [O(n)]
    fn is_hexagon(n: u32) -> bool {
        for i in hexagon() {
            if n == i {
                return true;
            } else if n > i {
                break;
            }
        }
        false
    }
    pub fn v1() -> u32 {
        let mut num: u32 = 0;
        for i in triangle() {
            if i == 40_755 {
                continue;
            }
            // i.e. 'next one'
            else if is_pentagon(i) && is_hexagon(i) {
                num = i;
                break;
            }
        }
        num
    }
}

pub mod p046 {
    fn is_prime(n: u32) -> bool {
        let half = n / 2 + 1;
        for i in 2..half {
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
    pub fn v1() -> u32 {
        // Maybe write a short explanation
        let mut smallest_odd_comp: u32 = 0;
        for i in (33..).step_by(2) {
            for p in primes() {
                let mut equal: bool = false;
                if p > i {
                    smallest_odd_comp = i;
                    break;
                }
                for j in 1.. {
                    // for j in (1..).map(|x| x.pow(2)) {
                    let j = (j as u32).pow(2);
                    if 2 * j + p > i {
                        break;
                    } else if 2 * j + p == i {
                        equal = true;
                        break;
                    }
                }
                if equal {
                    break;
                }
            }
        }
        smallest_odd_comp
    }
}

pub mod p047 {
    fn is_prime(n: u32) -> bool {
        let half = n / 2 + 1;
        for i in 2..half {
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
                n = n / i;
            }
            if n == 1 {
                break;
            }
        }
        p_fact
    }
    fn no_double(v: &mut Vec<u32>) -> bool {
        let length = v.len();
        v.sort(); //   v.sort().dedup();  ===>  WHY NOT ? because it doesn't return anything I assume but still...
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
        v.sort(); //   v.sort().dedup();  ===>  WHY NOT ? because it doesn't return anything I assume but still...
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
        let half = n / 2 + 1;
        for i in 2..half {
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
                for i in (self.minimum..self.maximum).rev() {
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
