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

