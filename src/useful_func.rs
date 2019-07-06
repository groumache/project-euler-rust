pub mod prime_numbers {
    pub fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        let half = n / 2 + 1;
        for i in 2..half {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    pub fn primes_below(n: u32) -> Vec<u32> {
        let mut primes: Vec<u32> = Vec::new();
        for i in 2..n {
            if is_prime(i) {
                primes.push(i);
            }
        }
        primes
    }
    pub struct PrimesIter {
        minimum: u32,
        maximum: u32,
        no_max: bool,
    }
    impl Iterator for PrimesIter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            if self.minimum < 2 {
                self.minimum = 2;
            }
            if self.maximum < 2 {
                self.maximum = 2;
            }
            let mut i: u32 = self.minimum;
            loop {
                if !self.no_max && i >= self.maximum {
                    break;
                }
                if is_prime(i) {
                    self.minimum = i + 1;
                    return Some(i);
                }
                i += 1;
            }
            None
        }
    }
    impl DoubleEndedIterator for PrimesIter {
        fn next_back(&mut self) -> Option<u32> {
            if self.minimum < 2 {
                self.minimum = 2;
            }
            if self.maximum < 2 {
                self.maximum = 2;
            }
            let max = self.maximum;
            for i in (self.minimum..self.maximum).rev() {
                if is_prime(i) {
                    self.maximum = i;
                    break;
                }
            }
            if max == self.maximum {
                return None;
            }
            Some(self.maximum)
        }
    }
    pub fn primes_minmax(min: u32, max: u32) -> PrimesIter {
        PrimesIter {
            minimum: min,
            maximum: max,
            no_max: false,
        }
    }
    pub fn primes_inf() -> PrimesIter {
        PrimesIter {
            minimum: 1,
            maximum: 1,
            no_max: true,
        }
    }
    pub fn prime_factors(n: u32) -> Vec<u32> {
        let mut n = n;
        let mut p_fact: Vec<u32> = Vec::new();
        for i in primes_inf() {
            while n % i == 0 {
                n = n / i;
                if p_fact.len() == 0 || *p_fact.last().unwrap() != i {
                    p_fact.push(i);
                }
            }
            if n == 1 {
                break;
            }
        }
        p_fact
    }
}

pub mod digits_numbers {
    pub fn get_digits(n: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let length = (n as f64).log10() as u32 + 1;
        for i in 0..length {
            let base: u32 = 10;
            let digit: u32 = n / base.pow(i) % 10;
            digits.push(digit);
        }
        digits
    }
    pub fn get_number(digits: &Vec<u32>) -> u32 {
        let mut num: u32 = 0;
        let base: u32 = 10;
        for (i, d) in digits.iter().enumerate() {
            num += d * base.pow(i as u32);
        }
        num
    }
    pub fn no_double(v: &mut Vec<u32>) -> bool {
        let length = v.len();
        v.sort(); //   v.sort().dedup();  ===>  WHY NOT ? because it doesn't return anything I assume but still...
        v.dedup();
        if length != v.len() {
            return false;
        }
        true
    }
}

pub mod other_func {
    pub struct TriangleNum {
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
    pub fn triangle_inf() -> TriangleNum {
        TriangleNum { n: 1 }
    }
    // could check with a 'formula' [O(1)] instead of [O(n)]
    pub fn is_triangle(n: u32) -> bool {
        for i in triangle_inf() {
            if n == i {
                return true;
            } else if i > n {
                break;
            }
        }
        false
    }
    pub struct PentagonalNum {
        n: u32,
    }
    impl Iterator for PentagonalNum {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let n = self.n;
            self.n += 1;
            Some(n * (3 * n - 1) / 2)
        }
    }
    pub fn pentagon_inf() -> PentagonalNum {
        PentagonalNum { n: 1 }
    }
    // could check with a 'formula' [O(1)] instead of [O(n)]
    pub fn is_pentagon(n: u32) -> bool {
        for i in pentagon_inf() {
            if n == i {
                return true;
            } else if i > n {
                break;
            }
        }
        false
    }
    pub struct HexagonalNum {
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
    pub fn hexagon_inf() -> HexagonalNum {
        HexagonalNum { n: 1 }
    }
    // could check with a 'formula' [O(1)] instead of [O(n)]
    pub fn is_hexagon(n: u32) -> bool {
        for i in hexagon_inf() {
            if n == i {
                return true;
            } else if i > n {
                break;
            }
        }
        false
    }
    pub struct FibonnacciIter {
        curr: u32,
        next: u32,
    }
    impl Iterator for FibonnacciIter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let curr = self.curr;
            self.curr = self.next;
            self.next += curr;
            Some(curr)
        }
    }
    pub fn fibonnacci_inf() -> FibonnacciIter {
        FibonnacciIter { curr: 1, next: 2 }
    }
    pub fn factors(n: u32) -> Vec<u32> {
        let mut fact: Vec<u32> = Vec::new();
        for i in 1..n + 1 {
            if n % i == 0 {
                fact.push(i);
            }
        }
        fact
    }
}
