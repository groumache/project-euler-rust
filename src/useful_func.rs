// Useful functions to solve the Project Euler problems

pub mod prime_numbers {
    pub fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }

        let sqrt_n: u32 = (n as f64).sqrt() as u32;
        let divisor = (2 ..= sqrt_n).find(|x| n % x == 0);

        divisor == None
    }
    pub fn primes_below(n: u32) -> Vec<u32> {
        (2 .. n).filter(|x| is_prime(*x))
            .collect()
    }
    pub fn prime_factors(n: u32) -> Vec<u32> {
        (2 ..= n)
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
            let prime = (self.minimum .. self.maximum).find(|x| is_prime(*x));

            if prime != None {
                self.minimum = 1 + prime.unwrap();
            }

            prime
        }
    }
    impl DoubleEndedIterator for PrimesIter {
        fn next_back(&mut self) -> Option<u32> {
            let prime = (self.minimum .. self.maximum)
                .rev()
                .find(|x| is_prime(*x));

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
        other_func::is_square(5*n.pow(2) + 4)
            || other_func::is_square(5*n.pow(2) - 4)
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

        let curr: u32 = (min ..).find(|x| is_fibonacci(*x)).unwrap();
        let next: u32 = (curr+1 ..).find(|x| is_fibonacci(*x)).unwrap();

        FibonacciIter {
            maximum: max,
            curr: curr,
            next: next,
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
    pub fn num_to_digits(n: u32) -> Vec<u32> {
        let mut n = n;
        let mut digits: Vec<u32> = Vec::new();
        let length = 1 + (n as f64).log10() as u32;

        for _ in 0 .. length {
            digits.push(n % 10);
            n /= 10;
        }

        digits
    }
    pub fn digits_to_num(digits: &Vec<u32>) -> u32 {
        let mut n: u32 = 0;

        for d in digits.iter().rev() {
            n = n * 10 + d;
        }

        n
    }

    pub fn no_double(v: &mut Vec<u32>) -> bool {
        let length = v.len();

        v.sort();
        v.dedup();

        length == v.len()
    }
}

pub mod triangle_num {
    use super::other_func;

    // An integer n is triangular exactly if 8n + 1 is a square.
    pub fn is_triangle(n: u32) -> bool {
        n != 0 && other_func::is_square(8*n + 1)
    }

    pub struct TriangleIter {
        minimum: u32,
        maximum: u32,
    }
    impl Iterator for TriangleIter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let triangle = (self.minimum .. self.maximum)
                .find(|x| is_triangle(*x));

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
        let n: f64 = n as f64;
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
            let pentagon = (self.minimum .. self.maximum)
                .find(|x| is_pentagon(*x));

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
        let n: f64 = n as f64;
        let x: f64 = ((8.0 * n + 1.0).sqrt() + 1.0) / 4.0;
        x - x.floor() == 0.0
    }

    pub struct HexagonIter {
        minimum: u32,
        maximum: u32,
    }
    impl Iterator for HexagonIter {
        type Item = u32;
        fn next(& mut self) -> Option<u32> {
            let hexagon = (self.minimum .. self.maximum)
                .find(|x| is_hexagon(*x));

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
        let x = n as f64;
        x != 0.0 && x.sqrt() % 1.0 == 0.0
    }

    pub fn factors(n: u32) -> Vec<u32> {
        let mut fact: Vec<u32> = Vec::new();
        for i in 1 ..= n {
            if n % i == 0 {
                fact.push(i);
            }
        }
        fact
    }

    pub fn is_palindrome<T: PartialEq<>>(v: Vec<T>) -> bool {
        let half = v.len() / 2;
        v.iter().take(half).eq( v.iter().rev().take(half) )
    }
}
