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

