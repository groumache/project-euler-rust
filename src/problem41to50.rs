// Problem 41: Pandigital prime
//   What is the largest n-digit pandigital prime that exists?
pub mod p041 {
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

// Problem 42: Coded triangle numbers
//   Using words.txt, a 16K text file containing nearly two-thousand
//   common English words, how many are triangle words ?
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
            if n == i { return true; }
            else if n > i { break; }
        }
        false
    }
    pub fn v1() -> u32 {
        let mut counter: u32 = 0;
        let filename = "p042_words.txt";
        let contents = fs::read_to_string(filename)
            .expect("Problem with reading the file");
        let words: Vec<&str> = contents.split(',').collect();
        for w in words {
            let w = w.to_string();
            let mut w_value: u32 = 0;       // w_value = w.iter().map(|l| letter_value(l)).collect()  ===>  LOL -> that would be cool but it's a bit extreme
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

// Problem 43: Sub-string divisibility
//   Find the sum of all 0 to 9 pandigital numbers with this property.
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
        if n == 0 { return 1; }
        (1..n+1).product()
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
        for i in (1..n_element+1).rev() {
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
            let mut items: Vec<u32> = [0,1,2,3,4,5,6,7,8,9].to_vec();
            items = get_permutation(items, self.perm).unwrap();
            Some(get_number(&items))
        }
    }
    fn pandigital() -> Pandigital10 {
        Pandigital10 { perm: 0, max: fact(10) }
    }
    pub fn v1() -> u32 {
        let mut sum_pandigital: u32 = 0;
        for i in pandigital() { // this could be much more simple
            let mut digits: Vec<u32> = get_digits(i);
            digits.reverse();
            let mut d234: Vec<u32> = Vec::new(); d234.push(digits[1]);
            d234.push(digits[2]); d234.push(digits[3]);
            let mut d345: Vec<u32> = Vec::new(); d345.push(digits[2]);
            d345.push(digits[3]); d345.push(digits[4]);
            let mut d456: Vec<u32> = Vec::new(); d456.push(digits[3]);
            d456.push(digits[4]); d456.push(digits[5]);
            let mut d567: Vec<u32> = Vec::new(); d567.push(digits[4]);
            d567.push(digits[5]); d567.push(digits[6]);
            let mut d678: Vec<u32> = Vec::new(); d678.push(digits[5]);
            d678.push(digits[6]); d678.push(digits[7]);
            let mut d789: Vec<u32> = Vec::new(); d789.push(digits[6]);
            d789.push(digits[7]); d789.push(digits[8]);
            let mut d8910: Vec<u32> = Vec::new(); d8910.push(digits[7]);
            d8910.push(digits[8]);
            if digits.len() > 9 { d8910.push(digits[9]); }

            if get_number(&d234) % 2 == 0 && get_number(&d345) % 3 == 0
                && get_number(&d456) % 5 == 0 && get_number(&d567) % 7 == 0
                && get_number(&d678) % 11 == 0 && get_number(&d789) % 13 == 0
                && get_number(&d8910) % 17 == 0 {
                sum_pandigital += i;
            }
        }
        sum_pandigital
    }
}

// Problem 44: Pentagon numbers
//   Find the pair of pentagonal numbers, Pj and Pk, for which their sum
//   and difference are pentagonal and D = |Pk − Pj| is minimised; what
//   is the value of D?
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
        PentagonNum { n: 1, }
    }
    // could check with a 'formula' [O(1)] instead of [O(n)]
    fn is_pentagon(n: u32) -> bool {
        for i in pentagon() {
            if n == i { return true; }
            else if n > i { break; }
        }
        false
    }
    pub fn v1() -> (u32, u32) {
        let mut pentagonal_num: (u32, u32) = (0, 0);
        for i in pentagon() {
            for j in pentagon() {
                if j >= i { break; }
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

