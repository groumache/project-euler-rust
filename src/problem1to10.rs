// Problem 1: Multiples of 3 and 5
//  Find the sum of all the multiples of 3 or 5 below 1000.
pub mod p001 {
    // Functionnal programming -- O(n)
    //  (3 + 6 + ...) + (5 + 10 + ...).except(x % 3 == 0)
    pub fn v1(limit: i32) -> i32 {
        let sum3: i32 = (3..limit).step_by(3).sum();
        let sum5: i32 = (5..limit).filter(|x| x % 3 != 0).step_by(5).sum();
        sum3 + sum5
    }
    // Math formula -- O(1)
    //  (3 + 6 + ...) + (5 + 10 + ...) - (15 + 30 + ...)
    pub fn v2(limit: i32) -> i32 {
        let nb3 = (limit - 1) / 3;
        let sum3 = 3 * nb3 * (1 + nb3) / 2; 
        let nb5 = (limit - 1) / 5;
        let sum5 = 5 * nb5 * (1 + nb5) / 2;
        let nb3x5 = limit / (3 * 5);
        let sum3x5 = (3 * 5) * nb3x5 * (1 + nb3x5) / 2;
        sum3 + sum5 - sum3x5
    }
}

// Problem 2: Even Fibonacci numbers
//  Find the sum of the even-valued terms.
pub mod p002 {
    // Fibonnacci iterations -- O(n)
    pub fn v1(nb_terms: i32) -> i32 {
        let mut curr = 1;
        let mut next = 2;
        let mut even_sum = 0;
        for _ in 1..nb_terms {
            let tmp = next;
            next = curr + next;
            curr = tmp;
            if curr % 2 == 0 {
                even_sum = even_sum + curr;
            }
        }
        even_sum
    }
}

// Problem 3: Largest prime factor
//  Find the largest prime factor.
pub mod p003 {
    // (1.) find factors (2.) find primes (3.) find largest
    pub fn v1(number: i32) -> i32 {
        let divisors: Vec<i32> = (2..number+1)
            .filter(|x| number % x == 0)
            .collect();
        let factors: Vec<i32> = divisors.clone();
        let mut prime: Vec<bool> = vec!(true; divisors.len());
        for (i, div) in divisors.iter().enumerate() {
            for x in factors.iter() {
                if div % x == 0 && div != x { prime[i] = false; }
            }
        }
        let mut prime_fact: i32 = 0;
        for (i, div) in divisors.iter().enumerate() {
            if prime[i] {
                prime_fact = *div;
            }
        }
        prime_fact
    }
    // (1.) find factors (2.) find largest prime
    pub fn v2(number: i32) -> i32 {
        let divisors: Vec<i32> = (2..number+1)
            .filter(|x| number % x == 0)
            .rev()
            .collect();
        let factors: Vec<i32> = divisors.clone();
        let mut prime_fact = 0;
        for div in divisors.iter() {
            if factors.iter().all(|x| div % x != 0 || div == x) {
                prime_fact = *div;
                break;
            }
        }
        prime_fact
    }
    // (1.) find primes (2.) find largest factor
    pub fn v3(number: i32) -> i32 {
        let mut sieve: Vec<bool> = vec!(true; (number+1) as usize);
        let num = number as usize;
        let half = num / 2 + 1;
        for i in 2..half {
            if sieve[i] {
                for j in (2*i..num+1).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
        let mut prime_fact = 0;
        for i in (2..num+1).rev() {
            if sieve[i] && num % i == 0 {
                prime_fact = i;
                break;
            }
        }
        prime_fact as i32
    }
}

// Problem 4: Largest palindrome product
//  Find the largest palindrome made from the product of two 3-digit numbers.
pub mod p004 {
    pub fn v1(nb_digits_factors: i32) -> i32 {
        let base = 10 as i32;
        let exp = (nb_digits_factors - 1) as u32;
        let min = base.pow(exp);
        let max = base.pow(exp+1);
        let mut largest_palyndrome = 0;
        for i in min..max {
            for j in min..i+1 {
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
