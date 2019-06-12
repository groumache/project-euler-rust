// Problem 1: Multiples of 3 and 5
//  Find the sum of all the multiples of 3 or 5 below 1000.
pub mod p001 {
    pub fn version1(limit: i32) -> i32 {    // O(n)
        let sum3: i32 = (3..limit).step_by(3).sum();
        let sum5: i32 = (5..limit).filter(|x| x % 3 != 0).step_by(5).sum();
        sum3 + sum5
    }
    pub fn version2(limit: i32) -> i32 {    // O(1)
        let max3 = (limit - 1) / 3;
        let max5 = (limit - 1) / 5;
        let max3x5 = limit / (3 * 5);
        let sum3 = 3 * max3 * (1 + max3) / 2; 
        let sum5 = 5 * max5 * (1 + max5) / 2;
        let sum3x5 = (3 * 5) * max3x5 * (1 + max3x5) / 2;
        sum3 + sum5 - sum3x5
    }
}
