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

