// Problem 1: Multiples of 3 and 5
//   Find the sum of all the multiples of 3 or 5 below 1000.
// Problem 2: Even Fibonacci numbers
//   Find the sum of the even-valued terms.
// Problem 3: Largest prime factor
//   Find the largest prime factor of n.
// Problem 4: Largest palindrome product
//   Find the largest palindrome made from the product of two 3-digit numbers.
// Problem 5: Smallest multiple
//   Smallest positive number that is divisible by all of the numbers from 1 to n?
// Problem 6: Sum Square Difference
//   Find the difference between the sum of the squares and the square of the sum of
//   the numbers from 1 to n.
// Problem 7: n-th prime
//   Find the n-th prime number
// Problem 8: Largest product in a series
//   Find the n adjacent digits that have the greatest product.
// Problem 9: Special Pythagorean triplet
//   There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//   Find the product abc.
// Problem 10: Summation of primes
//   Find the sum of all the primes below n

pub mod p001 {
    pub fn v1(n: u32) -> u32 {
        let sum_3: u32 = (3 .. n).step_by(3).sum();
        let sum_5: u32 = (5 .. n).filter(|x| x % 3 != 0).step_by(5).sum();

        sum_3 + sum_5
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

pub mod p002 {
    use crate::useful_func::fibonacci::fibonacci;
    pub fn v1(n: u32) -> u32 {
        let n: usize = n as usize;
        let mut sum: u32 = 0;
        
        for (i, fib) in fibonacci().enumerate() {
            if i == n {
                break;
            }
            
            if fib % 2 == 0 {
                sum += fib;
            }
        }
        sum
    }
}

pub mod p003 {
    use crate::useful_func::prime_numbers::*;
    pub fn v6(n: u32) -> u32 {
        let p_factors = prime_factors(n);
        *p_factors.iter().max().unwrap()
    }
}

pub mod p004 {
    // (1.) loop to find product (2.) check if palyndrome
    pub fn v1(nb_digits_factors: u32) -> u32 {
        let base = 10 as u32;
        let exp = (nb_digits_factors - 1) as u32;
        let min = base.pow(exp); // let min = 10_u32.pow(exp);
        let max = base.pow(exp + 1);
        let mut largest_palyndrome = 0;
        for i in min .. max {
            for j in min ..= i {
                let prod = j * i;
                let prod_str: String = prod.to_string();
                let prod_rev: String = prod_str.chars().rev().collect();
                if prod_str == prod_rev {
                    largest_palyndrome = prod;
                }
            }
        }
        largest_palyndrome
    }
}

pub mod p005 {
    //
    pub fn v1(n: u32) -> u32 {
        let n = n as usize;
        let half = n / 2;
        let mut sieve: Vec<bool> = vec![true; n + 1];
        for i in 2 ..= half {
            if sieve[i] {
                for j in (2*i ..= n).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
        let mut smallest_product = 1;
        for i in 2 ..= n {
            if sieve[i] {
                let repeat = (n as f64).log(i as f64);
                smallest_product *= i.pow(repeat as u32);
            }
        }
        smallest_product as u32
    }
}

pub mod p006 {
    pub fn v1(n: u32) -> u32 {
        let n: i32 = n as i32;
        let sum_square: i32 = (1 ..= n).map(|x| x * x).sum();
        let sum: i32 = (1 ..= n).sum();
        (sum_square - sum.pow(2)).abs() as u32
    }
}

pub mod p007 {
    pub fn v1(n: u32) -> u32 {
        let mut counter = 1;
        let mut number = 2;
        while counter != n {
            number += 1;
            // if number.isprime()
            if (2 .. number).all(|x| number % x != 0) {
                counter += 1;
            }
        }
        number
    }
}

pub mod p008 {
    fn substring(str: &String, start: usize, len: usize) -> String {
        let mut sub_string: String = String::new();
        for (i, c) in str.chars().enumerate() {
            if i >= start && i <= start + len - 1 {
                sub_string.push(c);
            }
        }
        sub_string
    }
    pub fn v1(window_size: usize) -> u32 {
        let num = String::from("73167176531330624919225119674426574742355349194934")
            + "96983520312774506326239578318016984801869478851843"
            + "85861560789112949495459501737958331952853208805511"
            + "12540698747158523863050715693290963295227443043557"
            + "66896648950445244523161731856403098711121722383113"
            + "62229893423380308135336276614282806444486645238749"
            + "30358907296290491560440772390713810515859307960866"
            + "70172427121883998797908792274921901699720888093776"
            + "65727333001053367881220235421809751254540594752243"
            + "52584907711670556013604839586446706324415722155397"
            + "53697817977846174064955149290862569321978468622482"
            + "83972241375657056057490261407972968652414535100474"
            + "82166370484403199890008895243450658541227588666881"
            + "16427171479924442928230863465674813919123162824586"
            + "17866458359124566529476545682848912883142607690042"
            + "24219022671055626321111109370544217506941658960408"
            + "07198403850962455444362981230987879927244284909188"
            + "84580156166097919133875499200524063689912560717606"
            + "05886116467109405077541002256983155200055935729725"
            + "71636269561882670428252483600823257530420752963450";
        let size = num.chars().count();
        let mut adjacents: Vec<Vec<u32>> = vec![Vec::new(); size - window_size];
        for i in 0 .. (size - window_size) {
            let adj: String = substring(&num, i, window_size);
            for c in adj.chars() {
                adjacents[i].push(c.to_digit(10).unwrap());
            }
        }
        let mut max_prod = 0;
        for adj in adjacents.iter() {
            let mut product: u32 = 1;
            for i in adj.iter() {
                product *= i;
            }
            if product > max_prod {
                max_prod = product;
            }
        }
        max_prod
    }
}

pub mod p009 {
    // iterate over a, b, c
    pub fn v1() -> (u32, u32, u32) {
        let mut a: u32 = 1;
        let mut b: u32 = 1;
        let mut c: u32 = 1;
        loop {
            if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1000 {
                break;
            }
            if a == b && b == c {
                c += 1;
                b = 1;
                a = 1;
            } else if b == a {
                b += 1;
                a = 1;
            } else {
                a += 1;
            }
        }
        (a, b, c)
    }
    // iterate over a, b
    pub fn v2() -> (u32, u32, u32) {
        let mut a: u32 = 1;
        let mut b: u32 = 1;
        let mut c: u32 = 1;
        loop {
            if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1000 {
                break;
            }
            if b == a {
                b += 1;
                a = 1;
            } else {
                a += 1;
            }
            c = ((a.pow(2) + b.pow(2)) as f64).sqrt() as u32;
        }
        (a, b, c)
    }
}

pub mod p010 {
    pub fn v1(n: u32) -> u32 {
        let n = n as usize;
        let mut sieve: Vec<bool> = vec![true; n + 1];
        let half = n / 2;
        for i in 2 ..= half {
            if sieve[i] {
                for j in (2*i ..= n).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
        let mut sum = 0;
        for i in 2 ..= n {
            if sieve[i] {
                sum += i;
            }
        }
        sum as u32
    }
}
