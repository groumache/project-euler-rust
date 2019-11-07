// Useful functions to solve the Project Euler problems

pub mod prime_numbers {
    pub fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }

        let sqrt_n: u32 = (f64::from(n)).sqrt() as u32;
        let divisor = (2..=sqrt_n).find(|x| n % x == 0);

        divisor == None
    }
    // when we remove the digit on the right, it stays a prime
    pub fn is_right_truncable_prime(n: u32) -> bool {
        if n <= 10 {
            return false;
        }

        let n_len = 1 + f64::from(n).log10() as u32;
        (0..n_len).all(|i| is_prime(n / 10_u32.pow(i as u32)))
    }
    // when we remove the digit on the left, it stays a prime
    pub fn is_left_truncable_prime(n: u32) -> bool {
        if n <= 10 {
            return false;
        }

        let n_len = 1 + f64::from(n).log10() as u32;
        (1..=n_len).all(|i| is_prime(n % 10_u32.pow(i as u32)))
    }
    pub fn primes_below(n: u32) -> Vec<u32> {
        (2..n).filter(|x| is_prime(*x)).collect()
    }
    pub fn prime_factors(n: u32) -> Vec<u32> {
        (2..=n)
            .filter(|x| n % x == 0)
            .filter(|x| is_prime(*x))
            .collect()
    }

    pub struct PrimesIter {
        minimum: u32,
        maximum: u32,
    }
    impl Iterator for PrimesIter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let prime = (self.minimum..self.maximum).find(|x| is_prime(*x));

            if prime != None {
                self.minimum = 1 + prime.unwrap();
            }

            prime
        }
    }
    impl DoubleEndedIterator for PrimesIter {
        fn next_back(&mut self) -> Option<u32> {
            let prime = (self.minimum..self.maximum).rev().find(|x| is_prime(*x));

            if prime != None {
                self.maximum = prime.unwrap();
            }

            prime
        }
    }

    pub fn primes_range(mut min: u32, mut max: u32) -> PrimesIter {
        if min < 2 {
            min = 2;
        }
        if max < min {
            max = min;
        }

        PrimesIter {
            minimum: min,
            maximum: max,
        }
    }
    pub fn primes() -> PrimesIter {
        PrimesIter {
            minimum: 2,
            maximum: u32::max_value(),
        }
    }
}

pub mod fibonacci {
    use super::other_func;

    // n is a fibonacci number iff (5*n^2+4) or (5*n^2-4) is a square
    pub fn is_fibonacci(n: u32) -> bool {
        other_func::is_square(5 * n.pow(2) + 4) || other_func::is_square(5 * n.pow(2) - 4)
    }

    pub struct FibonacciIter {
        maximum: u32,
        curr: u32,
        next: u32,
    }
    impl Iterator for FibonacciIter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let fib = self.curr;

            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;

            if fib >= self.maximum {
                None
            } else {
                Some(fib)
            }
        }
    }

    pub fn fibonacci_range(mut min: u32, mut max: u32) -> FibonacciIter {
        if min < 1 {
            min = 1;
        }
        if max < 2 {
            min = 1;
            max = 1;
        }

        let curr: u32 = (min..).find(|x| is_fibonacci(*x)).unwrap();
        let next: u32 = (curr + 1..).find(|x| is_fibonacci(*x)).unwrap();

        FibonacciIter {
            maximum: max,
            curr,
            next,
        }
    }
    pub fn fibonacci() -> FibonacciIter {
        FibonacciIter {
            maximum: u32::max_value(),
            curr: 1,
            next: 2,
        }
    }
}

pub mod digits {
    // return: Vec<digits>, in any given base
    fn num_to_digits_base(mut n: u32, base: u32) -> Vec<u32> {
        let mut digits: Vec<u32> = Vec::new();
        let n_len = 1 + f64::from(n).log(base as f64) as u32;

        for _ in 0..n_len {
            digits.push(n % base);
            n /= base;
        }

        digits
    }
    pub fn num_to_digits(n: u32) -> Vec<u32> {
        num_to_digits_base(n, 10)
    }
    pub fn num_to_binary_digits(n: u32) -> Vec<u32> {
        num_to_digits_base(n, 2)
    }
    pub fn digits_to_num(digits: &[u32]) -> u32 {
        let mut n: u32 = 0;

        for d in digits.iter().rev() {
            n = n * 10 + d;
        }

        n
    }

    pub fn frac_digits(numerator: u32, denominator: u32, length: usize) -> Vec<u32> {
        let den: f64 = denominator as f64;
        let mut digits: Vec<u32> = Vec::with_capacity(length);
        let start: i32 = ((numerator as f64) / (denominator as f64))
            .log10()
            .floor()
            .abs() as i32;
        let mut num: f64 = (numerator as f64) * 10_f64.powi(start);

        for _ in 0..length {
            let rem = num % den;
            let d: u32 = (num / den) as u32;

            println!("start = {}", start);

            digits.push(d as u32);
            num = rem * 10_f64;
        }

        digits
    }

    pub fn no_double(v: &mut Vec<u32>) -> bool {
        let length = v.len();

        v.sort();
        v.dedup();

        length == v.len()
    }

    pub fn get_grid_digits(grid: &str, width: usize) -> Vec<Vec<u32>> {
        let mut num_grid: Vec<Vec<u32>> = Vec::new();

        for (i, num) in grid.split_whitespace().enumerate() {
            if i % width == 0 {
                num_grid.push(Vec::new())
            }

            let last_index = num_grid.len() - 1;
            num_grid[last_index].push(num.parse::<u32>().unwrap());
        }

        num_grid
    }
    pub fn get_triangle_digits(triangle: &str) -> Vec<Vec<u32>> {
        use super::triangle_num::is_triangle;
        let mut num_triangle: Vec<Vec<u32>> = Vec::new();
        num_triangle.push(Vec::new());

        for (i, num) in triangle.split_whitespace().enumerate() {
            if is_triangle(i as u32) {
                num_triangle.push(Vec::new());
            }

            let last_index = num_triangle.len() - 1;
            num_triangle[last_index].push(num.parse::<u32>().unwrap());
        }

        num_triangle
    }
}

pub mod triangle_num {
    use super::other_func;

    // An integer n is triangular exactly if 8n + 1 is a square.
    pub fn is_triangle(n: u32) -> bool {
        n != 0 && other_func::is_square(8 * n + 1)
    }

    pub struct TriangleIter {
        minimum: u32,
        maximum: u32,
    }
    impl Iterator for TriangleIter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let triangle = (self.minimum..self.maximum).find(|x| is_triangle(*x));

            if triangle != None {
                self.minimum = 1 + triangle.unwrap();
            }

            triangle
        }
    }

    pub fn triangles_range(min: u32, max: u32) -> TriangleIter {
        TriangleIter {
            minimum: min,
            maximum: max,
        }
    }
    pub fn triangles() -> TriangleIter {
        TriangleIter {
            minimum: 1,
            maximum: u32::max_value(),
        }
    }
}

pub mod pentagonal_num {
    // n is a pentagonal number only if sqrt((24*n+1) + 1)/6 is an integer
    pub fn is_pentagon(n: u32) -> bool {
        let n: f64 = f64::from(n);
        let x: f64 = ((24.0 * n + 1.0).sqrt() + 1.0) / 6.0;
        x - x.floor() == 0.0
    }

    pub struct PentagonIter {
        minimum: u32,
        maximum: u32,
    }
    impl Iterator for PentagonIter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let pentagon = (self.minimum..self.maximum).find(|x| is_pentagon(*x));

            if pentagon != None {
                self.minimum = 1 + pentagon.unwrap();
            }

            pentagon
        }
    }

    pub fn pentagons_range(min: u32, max: u32) -> PentagonIter {
        PentagonIter {
            minimum: min,
            maximum: max,
        }
    }
    pub fn pentagons() -> PentagonIter {
        PentagonIter {
            minimum: 1,
            maximum: u32::max_value(),
        }
    }
}

pub mod hexagonal_num {
    // n is a hexagonal number if (sqrt(8*y+1) + 1)/4 is an integer
    pub fn is_hexagon(n: u32) -> bool {
        let n: f64 = f64::from(n);
        let x: f64 = ((8.0 * n + 1.0).sqrt() + 1.0) / 4.0;
        x - x.floor() == 0.0
    }

    pub struct HexagonIter {
        minimum: u32,
        maximum: u32,
    }
    impl Iterator for HexagonIter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let hexagon = (self.minimum..self.maximum).find(|x| is_hexagon(*x));

            if hexagon != None {
                self.minimum = 1 + hexagon.unwrap();
            }

            hexagon
        }
    }

    pub fn hexagons_range(min: u32, max: u32) -> HexagonIter {
        HexagonIter {
            minimum: min,
            maximum: max,
        }
    }
    pub fn hexagons() -> HexagonIter {
        HexagonIter {
            minimum: 1,
            maximum: u32::max_value(),
        }
    }
}

pub mod other_func {
    pub fn is_square(n: u32) -> bool {
        let x = f64::from(n);
        x != 0.0 && x.sqrt() % 1.0 == 0.0
    }
    pub fn is_amicable(a: u32) -> bool {
        let b: u32 = factors(a).iter().sum::<u32>() - a;
        let new_a: u32 = factors(b).iter().sum::<u32>() - b;

        a != b && a == new_a
    }
    pub fn is_perfect(n: u32) -> bool {
        2 * n == factors(n).iter().sum::<u32>()
    }
    pub fn is_deficient(n: u32) -> bool {
        2 * n > factors(n).iter().sum::<u32>()
    }
    pub fn is_abundant(n: u32) -> bool {
        2 * n < factors(n).iter().sum::<u32>()
    }
    pub fn is_circular_prime(n: u32) -> bool {
        use crate::useful_func::digits::{digits_to_num, num_to_digits};
        use crate::useful_func::prime_numbers::is_prime;

        let mut digits: Vec<u32> = num_to_digits(n);
        let n_len = digits.len();

        (0..n_len).all(|_| {
            digits.rotate_right(1);
            is_prime(digits_to_num(&digits))
        })
    }

    pub fn factors(n: u32) -> Vec<u32> {
        (1..=n).filter(|i| n % i == 0).collect()
    }

    pub fn fact(n: u32) -> u32 {
        (1..=n).product()
    }

    pub fn get_gcd(mut n1: u32, mut n2: u32) -> u32 {
        use crate::useful_func::prime_numbers::primes_range;
        use std::cmp::min;

        let mut gcd: u32 = 1;

        for p in primes_range(0, min(n1, n2)) {
            while n1 % p == 0 && n2 % p == 0 {
                n1 = n1 / p;
                n2 = n2 / p;

                gcd *= p;
            }
        }

        gcd
    }

    pub fn is_palindrome<T: PartialEq>(v: Vec<T>) -> bool {
        let half = v.len() / 2;
        v.iter().take(half).eq(v.iter().rev().take(half))
    }

    pub fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
        a < b && b < c && a.pow(2) + b.pow(2) == c.pow(2)
    }

    pub fn get_substring(string: &str, start: usize, len: usize) -> String {
        let end = start + len;
        string.get(start..end).unwrap().to_string()
    }
}

pub mod coins {
    pub struct Coins {
        pub pound2: u32,
        pub pound1: u32,
        pub p50: u32,
        pub p20: u32,
        pub p10: u32,
        pub p5: u32,
        pub p2: u32,
        pub p1: u32,
    }

    impl PartialEq for Coins {
        fn eq(&self, other: &Self) -> bool {
            self.pound2 == other.pound2
                && self.pound1 == other.pound1
                && self.p50 == other.p50
                && self.p20 == other.p20
                && self.p10 == other.p10
                && self.p5 == other.p5
                && self.p2 == other.p2
                && self.p1 == other.p1
        }
    }
}
