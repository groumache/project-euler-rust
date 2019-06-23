// Problem: Coin sums
//   How many different ways can £2 be made using any number of coins ?
pub mod p031 {
    struct Coins {
        pound2: u8,
        pound1: u8,
        p50: u8,
        p20: u8,
        p10: u8,
        p5: u8,
        p2: u8,
        p1: u8,
    }
    impl PartialEq for Coins {
        fn eq(&self, other: &Self) -> bool {
            self.pound2 == other.pound2 && self.pound1 == other.pound1
                && self.p50 == other.p50 && self.p20 == other.p20
                && self.p10 == other.p10 && self.p5 == other.p5
                && self.p2 == other.p2 && self.p1 == other.p1
        }
    }
    // We just try everything in an orderly manner.
    pub fn v1() -> u32 {
        let mut counter: u32= 1;
        let mut curr: Coins = Coins { pound2: 1, pound1: 0, p50: 0, p20: 0, p10: 0, p5: 0, p2: 0, p1: 0 };
        let terminal = Coins { pound2: 0, pound1: 0, p50: 0, p20: 0, p10: 0, p5: 0, p2: 0, p1: 200 };
        while curr != terminal {
            counter += 1;
            if curr.p2 != 0 {
                curr.p1 += 2;
                curr.p2 -= 1;
                continue;
            } else if curr.p5 != 0 {
                curr.p1 += 1;
                curr.p2 += 2;
                curr.p5 -= 1;
                continue;
            } else if curr.p10 != 0 {
                curr.p5 += 2;
                curr.p10 -= 1;
                continue;
            } else if curr.p20 != 0 {
                curr.p10 += 2;
                curr.p20 -= 1;
                continue;
            } else if curr.p50 !=0 {
                curr.p10 += 1;
                curr.p20 += 2;
                curr.p50 -= 1;
                continue;
            } else if curr.pound1 != 0 {
                curr.p50 += 2;
                curr.pound1 -= 1;
                continue;
            } else if curr.pound2 != 0 {
                curr.pound1 += 2;
                curr.pound2 -= 1;
                continue;
            }
        }
        counter
    }
}

