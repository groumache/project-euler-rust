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
    // (1.) loop to find product (2.) check if palyndrome
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

// Problem 5: Smallest multiple
//  Smallest positive number that is divisible by all of the numbers from 1 to 'n'?
pub mod p005 {
    // 
    pub fn v1(n: i32) -> i32 {
        let n = n as usize;
        let half = n / 2 + 1;
        let mut sieve: Vec<bool> = vec!(true; n+1);
        for i in 2..half {
            if sieve[i] {
                for j in (2*i..n+1).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
        let mut smallest_product = 1;
        for i in 2..n+1 {
            if sieve[i] {
                let repeat = (n as f64).log(i as f64);
                smallest_product *= i.pow(repeat as u32);
            }
        }
        smallest_product as i32
    }
}

// Problem 6: Sum Square Difference
//  Find the difference between the sum of the squares and the square of the sum of
//  the numbers from 1 to 'n'.
pub mod p006 {
    pub fn v1(n: i32) -> i32 {
        let sum_square: i32 = (1..n+1).map(|x| x*x).sum();
        let sum: i32 = (1..n+1).sum();
        (sum_square - sum.pow(2)).abs()
    }
}

// Problem 7: n-th prime
//  Find the n-th prime number
pub mod p007 {
    pub fn v1(n: i32) -> i32 {
        let mut counter = 1;
        let mut number = 2;
        while counter != n {
            number += 1;
            // if number.isprime()
            if (2..number).all(|x| number % x != 0) {
                counter += 1;
            }
        }
        number
    }
}

// Problem 8: Largest product in a series
//  Find the 'n' adjacent digits that have the greatest product.
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
        let mut adjacents: Vec<Vec<u32>> = vec!(Vec::new(); size - window_size);
        for i in 0..(size - window_size) {
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

