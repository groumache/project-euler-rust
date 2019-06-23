// Problem 21: Amicable numbers
//   Evaluate the sum of all the amicable numbers under n.
pub mod p021 {
    pub fn v1(n: u32) -> u32 {
        let divisors = |n: u32| -> Vec<u32> {
            let half = n / 2 + 1;
            let mut div: Vec<u32> = Vec::new();
            for i in 1..half {
                if n % i == 0 { div.push(i); }
            }
            div
        };
        let d = |n: u32| -> u32 { // 'd' as defined in the statement
            divisors(n).iter().sum()
        };
        let mut amicable_numbers: Vec<u32> = Vec::new();
        for i in 1..n {
            if d(i) == d(d(i)) && !amicable_numbers.contains(&i) {
                amicable_numbers.push(i);
            }
        }
        amicable_numbers.iter().sum()
    }
}

// Problem 22: Names scores
//   What is the total of all the name scores in the file?
pub mod p022 {
    use std::fs;
    pub fn v1() -> u32 {
        let filename = "p022_names.txt";
        let contents = fs::read_to_string(filename)
            .expect("Problem with reading the file");
        let vec_names: Vec<&str> = contents.split(',').collect();
        // remove quote (") from the names
        //for name in vec_names { name = name.chars().filter(|c| *c == '"').collect(); }
        let mut vec_names: Vec<String> = vec_names.iter()
            .map(|s| s
                .to_string()
                .chars()
                .filter(|c| *c != '"')
                .collect())
            .collect();
        vec_names.sort();               // I don't understand why I can't just write '.sort()' just above this line (below '.collect()')  -->  Try to figure this out later.
        let letter_value = |l: char| -> u32 {
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
        };
        let mut total_score: u32 = 0;
        for (i, name) in vec_names.iter().enumerate() {
            let mut score_name: u32 = 0;
            for c in name.chars() { score_name += letter_value(c); }
            let pos = (i + 1) as u32;
            total_score += score_name * pos;
        }
        total_score
    }
}

// Problem 23: Non-abundant sums
//   Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
pub mod p023 {
    pub fn v1() -> u32 {
        let divisors = |n: u32| -> Vec<u32> {
            let half = n / 2 + 1;
            let mut div: Vec<u32> = Vec::new();
            for i in 1..half {
                if n % i == 0 { div.push(i); }
            }
            div
        };
        let mut abundant_numbers: Vec<u32> = Vec::new();
        let mut sum_abundant_num: Vec<u32> = Vec::new();
        let mut not_sum_abundant_num: Vec<u32> = Vec::new();
        let limit: u32 = 28123 + 1; // it says "greater than", not "greater or equal"
        // find abundant numbers
        for i in 1..limit {
            if i < divisors(i).iter().sum() { abundant_numbers.push(i); }
        }
        // find the sum of each pair of abundant number
        for i in &abundant_numbers {
            for j in &abundant_numbers {
                sum_abundant_num.push(i + j);
            }
        }
        // find numbers that cannot be written as the sum of two abundant numbers
        //   can be replaced by:
        //   not_sum_abundant_num = (1..limit).filter(|n| !sum_abundant_num.contains(n)).collect();
        for i in 1..limit {
            if !sum_abundant_num.contains(&i) {
                not_sum_abundant_num.push(i);
            }
        }
        not_sum_abundant_num.iter().sum()
    }
}

// Problem 24: Lexicographic permutations
//   What is the millionth lexicographic permutation of the digits 
//   0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
pub mod p024 {
    // this version does not compute all the permutations but find
    // which elements should be shifted and where
    pub fn v1() -> Vec<u32> {
        /*
            0 1 2 3 4 5 6 7 8 9 -- len = 10
                --> 2 0 1 3 4 5 6 7 8 9, permutation n°(2 * 9!)
            0 1 3 4 5 6 7 8 9   -- len = 9
                --> 8 0 1 3 4 5 6 7 9, permutation n°(6 * 8! + 2 * 9!)
            ...
            1 000 000  / 9!  = 2.7
            1 000 000  % 9!  = 274 240
            274 240    / 8!  = 6.8
            274 240    % 8!  = 32 320
            ...
        */
        let n_element = 10;
        let mut perm = 1_000_000;
        let mut permutations: Vec<u32> = Vec::new();
        // find which element we have to shift
        for i in (1..n_element+1).rev() {
            let factorial: u32 = (1..i+1).product();
            permutations.push(perm / factorial);
            perm = perm % factorial;
        }
        // compute the permutations
        let mut elements: Vec<u32> = (0..10).collect();
        let mut final_perm: Vec<u32> = Vec::new();
        for i in permutations.iter() {
            final_perm.push(elements.remove(*i as usize));
        }
        final_perm
    }
}

// Problem 25: 1000-digit Fibonacci number
//   What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
pub mod p025 {
    pub fn v1() -> u32 {
        let mut counter = 3;
        let mut prev = 1;
        let mut curr = 2;
        loop {
            if (curr as f64).log10() >= 1000 as f64 {
                break;
            }
            let temp = curr;
            curr += prev;
            prev = temp;
            counter += 1;
        }
        counter
    }
}

// Problem 26: Reciprocal cycles
//   Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
pub mod p026 {
    fn all_primes_below(n: u32) -> Vec<u32> {
        let mut sieve: Vec<bool> = vec!(true; (n+1) as usize);
        let num = n as usize;
        let half = num / 2 + 1;
        //                                                                                      sieve[0..1] = false;
        sieve[0] = false;
        sieve[1] = false;
        for i in 2..half {
            if sieve[i] {
                for j in (2*i..num+1).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
        let mut list_primes: Vec<u32> = Vec::new();
        for (num, prime) in sieve.iter().enumerate() {
            if *prime { list_primes.push(num as u32); }
        }
        list_primes
    }
    // does not work if cycle possesses sub-cycles, i.e. [01 01 2] [01 01 2] ...
    pub fn v1() -> u32 {
        // largest period possible for 1/n is n-1 -- only need to check primes
        let n = 1000;
        let mut max_d = 0;
        let mut length_max_cycle = 0;
        let primes = all_primes_below(n);
        for i in primes {
            let frac: f64 = 1.0 / (i as f64); // frac = 1 / i   (why does it have to be so complicated...)
            let log = frac.log10() as u32;
            let mut digits: Vec<u8> = Vec::new();
            // we're supposed to observe a repetition of the cycle at least once
            for j in 1..2*n {
                let base: u32 = 10;
                let digit_position: f64 = base.pow(j - log) as f64;
                let digit: u8 = ((frac * digit_position) % 10 as f64) as u8;
                digits.push(digit);
            }
            // find cycle.length
            let mut cycle_length = 0;
            let last_digit: u8 = *digits.last().unwrap();
            let mut last_occurence = digits.len() - 1;
            for (j, digit) in digits.iter().rev().enumerate() {
                if *digit == last_digit && j < last_occurence { // find a second occurence of the digit
                    let mut eq_cycles = true;
                    for k in j..digits.len() { // check if that means that we've found a repetition of the cycle
                        if digits[k] != digits[j-k] {
                            eq_cycles = false;
                            break;
                        }
                    }
                    if eq_cycles {
                        cycle_length = digits.len() - j;
                        break;
                    } else {
                        last_occurence = j;
                    }
                }
            }
            if cycle_length > length_max_cycle {
                length_max_cycle = cycle_length;
                max_d = i;
            }
        }
        max_d
    }
}

// Problem 27: Quadratic primes
//   Find the product of the coefficients, a and b, for n^2 + a n + b,
//   where |a| < 1000 and |b| ≤ 1000 that produces the maximum number
//   of primes for consecutive values of n, starting with n = 0.
pub mod p027 {
    fn is_prime(n: u32) -> bool {
        let half = n / 2 + 1;
        for i in 2..half {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    pub fn v1() -> u32 {
        let max_coeff = 1000;
        let mut n_primes = 0;
        let mut max_prod = 0;
        for a in 0..max_coeff {
            for b in 0..max_coeff {
                let mut n: u32 = 0;
                while is_prime(n.pow(2) + a * n + b) {
                    n += 1;
                }
                if n > n_primes {
                    n_primes = n;
                    max_prod = a * b;
                }
            }
        }
        max_prod
    }
}

// Problem 28: Number spiral diagonals
//   What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral ?
pub mod p028 {
    enum Direction {      //      AN ENUM MIGHT BE BETTER  ===>  Direction::right  or  Direction::left  or  ...
        Right,
        Left,
        Up,
        Down,
    }
    pub fn v1() -> u32 {
        let mut sum: u32 = 0;
        let mut point: (i32, i32) = (0, 0);
        let mut dir: Direction = Direction::Right;
        let mut num: u32 = 1;
        while point != (500, 500) { // correspond to a 1001 x 1001 spiral
            if point.0 == -point.1 && point.0 <= 0 { dir = Direction::Right; }
            else if point.0 == -point.1 && point.0 >= 0 { dir = Direction::Left; }
            else if point.0 ==  point.1 && point.0 <= 0 { dir = Direction::Up; }
            else if (point.0 + 1) == point.1  { dir = Direction::Down; }

            if point.0.abs() == point.1.abs() { sum += num; }
            num += 1;

            match dir {
                Direction::Down => point.1 -= 1,
                Direction::Left => point.0 -= 1,
                Direction::Right => point.0 += 1,
                Direction::Up => point.1 += 1,
            };
        }
        sum + num // add the last element to the sum
    }
}

