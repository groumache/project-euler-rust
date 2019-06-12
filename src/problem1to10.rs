// Problem 1: Multiples of 3 and 5
//  Find the sum of all the multiples of 3 or 5 below 1000.
pub mod p001 {
    pub fn version1(limit: i32) -> i32 {    // O(n)
        let sum3: i32 = (3..limit).step_by(3).sum();
        let sum5: i32 = (5..limit).filter(|x| x % 3 != 0).step_by(5).sum();
        sum3 + sum5
    }
    pub fn version2(limit: i32) -> i32 {    // O(1)
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
    pub fn version1(nb_terms: i32) -> i32 {
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
