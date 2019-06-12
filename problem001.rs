// Problem 1: Multiples of 3 and 5
//  Find the sum of all the multiples of 3 or 5 below 1000.
pub mod prob1 {
    pub fn version1(limit: i32) -> i32 {
        let sum3: i32 = (3..limit).step_by(3).sum();
        let sum5: i32 = (5..limit).filter(|x| x % 3 != 0).step_by(5).sum();
        sum3 + sum5
    }
}
