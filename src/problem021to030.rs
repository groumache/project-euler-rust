// Find the sum of all the amicable numbers under n.
pub mod p021 {
    use crate::useful_func::other_func::is_amicable;

    pub fn v1(n: u32) -> u32 {
        (1 .. n).filter(|x| is_amicable(*x)).count() as u32
    }
}

// What is the total of all the name scores in the file?
pub mod p022 {
    use std::fs;
    pub fn v1() -> u32 {
        let filename = "p022_names.txt";
        let contents = fs::read_to_string(filename).expect("Problem with reading the file");
        let vec_names: Vec<&str> = contents.split(',').collect();
        // remove quote (") from the names
        //for name in vec_names { name = name.chars().filter(|c| *c == '"').collect(); }
        let mut vec_names: Vec<String> = vec_names
            .iter()
            .map(|s| s.to_string().chars().filter(|c| *c != '"').collect())
            .collect();
        vec_names.sort(); // I don't understand why I can't just write '.sort()' just above this line (below '.collect()')  -->  Try to figure this out later.
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
            for c in name.chars() {
                score_name += letter_value(c);
            }
            let pos = (i + 1) as u32;
            total_score += score_name * pos;
        }
        total_score
    }
}

// Find the sum of positive integers which cannot be written as the sum of two
// abundant numbers below n.
pub mod p023 {
    use crate::useful_func::other_func::is_abundant;

    pub fn v1(n: u32) -> u32 {
        let abundants: Vec<u32> = (1 .. n)
            .filter(|x| is_abundant(*x))
            .collect();

        let mut sum_abundants: Vec<u32> = Vec::new();
        for i in &abundants {
            for j in &abundants {
                if i + j >= n {
                    break;
                }
                sum_abundants.push(i + j);
            }
        }

        (1 .. n).filter(|x| !sum_abundants.contains(x)).count() as u32
    }
}

// Find the n-th lexicographic permutation of the digits
// 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
pub mod p024 {
    use crate::useful_func::other_func::fact;

    pub fn v1(mut perm: u32) -> Vec<u32> {
        let mut v_ini = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
        let mut v_fin = Vec::new();

        for i in (0 .. 10).rev() {
            let pos = (perm / fact(i)) as usize;
            perm = perm % fact(i);

            v_fin.push( v_ini.remove(pos) );
        }

        v_fin
    }
}

// What is the index of the first term in the Fibonacci sequence to contain n digits?
pub mod p025 {
    use crate::useful_func::fibonacci::fibonacci;

    pub fn v1(n: u32) -> u32 {
        let mut first = 0;
        let limit = 10_u32.pow(n - 1);

        for i in fibonacci() {
            if i >= limit {
                first = i;
                break;
            }
        }

        first
    }
}

// Find the value of p < n for which 1/p has the longest recurring cycle in its decimal fraction part.
pub mod p026 {
    use crate::useful_func::prime_numbers::primes_below;
    use crate::useful_func::digits::frac_digits;

    // largest period possible for 1/p is p-1 -- only need to check primes
    pub fn v1(n: u32) -> u32 {
        let primes = primes_below(n);
        let mut max_p = 0;
        let mut max_cycle = 0;

        for p in primes {
            let length: usize = p as usize;
            let mut digits = frac_digits(1, p, 3 * length);
            digits = digits.split_off(length);

            // find cycle length
            for i in 1 .. (p as usize) {
                let mut is_cycle = true;
                let mut start = i;
                let mut end = 2 * i;

                while end < digits.len() {
                    if digits[start .. end] != digits[(start - i) .. (end - i)] {
                        is_cycle = false;
                        break;
                    }

                    // update
                    start = end;
                    end += i;
                }

                if is_cycle && (i + 1) > max_cycle {
                    max_cycle = i + 1;
                    max_p = p;
                }
            }
        }

        max_p
    }
}


// Find a and b, for n^2 + a n + b, where |a| < 'max' and |b| <= 'max' that produces
// the maximum number of primes for consecutive values of n, starting with n = 0.
pub mod p027 {
    use crate::useful_func::prime_numbers::is_prime;

    pub fn v1(max: u32) -> u32 {
        let mut coeff_prod = 0;
        let mut max_n = 0;

        for a in 0 .. max {
            for b in 0 ..= max {
                let n_primes = (0_u32 ..)
                    .take_while(|n: &u32| is_prime(n.pow(2) + a * n + b))
                    .count();

                if n_primes > max_n {
                    max_n = n_primes;
                    coeff_prod = a * b;
                }
            }
        }

        coeff_prod
    }
}

// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral ?
pub mod p028 {
    enum Direction {
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

        while point != (500, 500) {
            // correspond to a 1001 x 1001 spiral
            if point.0 == -point.1 && point.0 <= 0 {
                dir = Direction::Right;
            } else if point.0 == -point.1 && point.0 >= 0 {
                dir = Direction::Left;
            } else if point.0 == point.1 && point.0 <= 0 {
                dir = Direction::Up;
            } else if (point.0 + 1) == point.1 {
                dir = Direction::Down;
            }

            if point.0.abs() == point.1.abs() {
                sum += num;
            }
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

// How many distinct terms are in the sequence generated by a^b
// for 2 ≤ a ≤ 'n' and 2 ≤ b ≤ 'n' ?
pub mod p029 {
    pub fn v1(n: u32) -> u32 {
        let mut sequence: Vec<u32> = Vec::new();
        
        for a in 2 ..= n as u32 {
            for b in 2 ..= n as u32 {
                let pow = a.pow(b);
        
                if !sequence.contains(&pow) {
                    sequence.push(pow);
                }
            }
        }
        
        sequence.len() as u32
    }
}

// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
pub mod p030 {
    use crate::useful_func::digits::num_to_digits;

    // max 5 digits because 5 * 9^5 > 99 999 and 6 * 9^6 < 999 999
    pub fn v1() -> u32 {
        (10 .. 100_000).filter(
            |x| *x == num_to_digits(*x).iter().map(|d| d.pow(5)).sum()
        ).sum()
    }
}
