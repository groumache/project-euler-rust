// Find the sum of all the multiples of 3 or 5 below n.
pub mod p001 {
    pub fn v1(n: u32) -> u32 {
        (3 .. n).filter(|x| x % 3 == 0 || x % 5 == 0)
            .sum()
    }

    // Math formula -- \sum_{i = 1}^{n} = n * (n + 1) / 2
    pub fn v2(n: u32) -> u32 {
        let max_3 = (n - 1) / 3;
        let max_5 = (n - 1) / 5;
        let max_3x5 = (n - 1) / (3 * 5);

        let sum_3 = 3 * max_3 * (max_3 + 1) / 2;
        let sum_5 = 5 * max_5 * (max_5 + 1) / 2;
        let sum_3x5 = (3 * 5) * max_3x5 * (max_3x5 + 1) / 2;

        sum_3 + sum_5 - sum_3x5
    }
}

// Find the sum of the even-valued Fibonacci terms below n.
pub mod p002 {
    use crate::useful_func::fibonacci::fibonacci;

    pub fn v1(n: u32) -> u32 {
        let n = n as usize;
        fibonacci().take(n).filter(|fib| *fib % 2 == 0).sum()
    }
}

// Find the largest prime factor of n.
pub mod p003 {
    use crate::useful_func::prime_numbers::prime_factors;

    pub fn v1(n: u32) -> u32 {
        *prime_factors(n).iter().max().unwrap()
    }
}

// Find the largest palindrome made from the product of two n-digit numbers.
pub mod p004 {
    use crate::useful_func::other_func::is_palindrome;
    use crate::useful_func::digits::num_to_digits;

    pub fn v1(n: u32) -> u32 {
        let min = 10_u32.pow(n-1);
        let max = 10_u32.pow(n);
        let mut palindrome: u32 = 0;

        'outer:
        for i in (min .. max).rev() {
            for j in (i .. max).rev() {
                palindrome = j * i;
                if is_palindrome(num_to_digits(palindrome)) {
                    break 'outer;
                }
            }
        }

        palindrome
    }
}

// Find the smallest positive number that is divisible by all of the numbers from 1 to n.
pub mod p005 {
    use crate::useful_func::prime_numbers::primes_below;

    pub fn v1(n: u32) -> u32 {
        let mut smallest_product: u32 = 1;

        for p in primes_below(n+1).iter() {
            let exp: u32 = (f64::from(n)).log(f64::from(*p)) as u32;
            smallest_product *= p.pow(exp);
        }

        smallest_product
    }
}

// Find the difference between the sum of the squares and the square of the sum of
// the numbers from 1 to n.
pub mod p006 {
    pub fn v1(n: u32) -> i32 {
        let n: i32 = n as i32;

        let sum_square: i32 = (1 ..= n).map(|x| x * x).sum();
        let sum: i32 = (1 ..= n).sum();
        let square_sum = sum.pow(2);

        (sum_square - square_sum).abs()
    }
}

// Find the n-th prime number.
pub mod p007 {
    use crate::useful_func::prime_numbers::primes;

    pub fn v1(n: usize) -> u32 {
        primes().nth(n-1).unwrap()
    }
}

// Find the n adjacent digits that have the greatest product.
pub mod p008 {
    use crate::useful_func::other_func::get_substring;
    use crate::useful_func::digits::num_to_digits;
    use std::cmp::max;

    pub fn v1(win_size: usize, num_string: &str) -> u32 {
        let mut max_prod: u32 = 0;
        let n = num_string.len() - win_size;

        for i in 0 .. n {
            let win_string = get_substring(&num_string, i, win_size)
                .parse::<u32>()
                .unwrap();
            let prod = num_to_digits(win_string).iter().product();

            max_prod = max(prod, max_prod);
        }

        max_prod
    }
}

// Find the product of the only pythagorean triplet for which a + b + c = 1000.
pub mod p009 {
    use crate::useful_func::other_func::is_pythagorean_triplet;

    // iterate over a, b
    pub fn v1() -> (u32, u32, u32) {
        let mut triplet: (u32, u32, u32) = (0, 0, 0);

        'outer:
        for a in 1 .. 1000_u32 {
            for b in a+1 .. 1000 {
                let c = ((a.pow(2) + b.pow(2)) as f64).sqrt() as u32;

                if a + b + c == 1000 && is_pythagorean_triplet(a, b, c) {
                    triplet = (a, b, c);
                    break 'outer;
                }
            }
        }

        triplet
    }
}

// Find the sum of all the primes below n.
pub mod p010 {
    use crate::useful_func::prime_numbers::primes_below;

    pub fn v1(n: u32) -> u32 {
        primes_below(n).iter().sum()
    }
}
