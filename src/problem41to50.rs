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

