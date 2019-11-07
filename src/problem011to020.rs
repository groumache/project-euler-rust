// Find the greatest product of 'n_adjacent' numbers in the same direction
// (up, down, left, right, or diagonally) in the 'width x width' grid.
pub mod p011 {
    use crate::useful_func::digits::get_grid_digits;
    use std::cmp::max;

    pub fn v1(grid: &str, width: usize, n_adj: usize) -> u32 {
        let num_grid: Vec<Vec<u32>> = get_grid_digits(grid, width);
        let height: usize = num_grid.len();
        let mut max_prod = 0;

        // Horizontal
        for row in &num_grid {
            for col in 0..(width - n_adj + 1) {
                let prod = row[col..(col + n_adj)].iter().product();

                max_prod = max(prod, max_prod);
            }
        }

        // Vertical
        for col in 0..width {
            for row in 0..(height - n_adj + 1) {
                // let prod = num_grid[row .. (row+n_adj)][col]
                //     .iter().product();
                let mut prod = 1;

                for i in 0..n_adj {
                    prod *= num_grid[row + i][col];
                }

                max_prod = max(prod, max_prod);
            }
        }

        // Diagonal 1
        for row in 0..(height - n_adj + 1) {
            for col in 0..(width - n_adj + 1) {
                let mut prod = 1;

                for i in 0..n_adj {
                    prod *= num_grid[row + i][col + i];
                }

                max_prod = max(prod, max_prod);
            }
        }

        // Diagonal 2
        for row in (n_adj - 1)..height {
            for col in 0..(width - n_adj + 1) {
                let mut prod = 1;

                for i in 0..n_adj {
                    prod *= num_grid[row - i][col + i];
                }

                max_prod = max(prod, max_prod);
            }
        }

        max_prod as u32
    }
}

// Find the first triangle number to have over 'n' divisors
pub mod p012 {
    use crate::useful_func::other_func::factors;
    use crate::useful_func::triangle_num::triangles;

    pub fn v1(n: u32) -> u32 {
        triangles()
            .find(|x| factors(*x).len() > (n as usize))
            .unwrap()
    }
}

// Find the first n digits of the sum of the list of numbers.
pub mod p013 {
    use crate::useful_func::other_func::get_substring;

    pub fn v1(vec_num: Vec<&str>, n_digits: usize) -> u64 {
        let mut sum: u64 = 0;

        for i in vec_num.iter() {
            let first = i.len() - n_digits;
            sum += get_substring(i, first, n_digits).parse::<u64>().unwrap();
        }

        sum % 10_u64.pow(10)
    }
}

// Which starting number, under 'n', produces the longest Collatz sequence?
pub mod p014 {
    pub fn v1(n: u32) -> u32 {
        let mut start_number = 1;
        let mut max_length = 1;

        for curr in 2..=n {
            let mut chain_length = 1;
            let mut i = curr;

            while i != 1 {
                if i % 2 == 0 {
                    i /= 2;
                    chain_length += 1;
                } else {
                    i = 3 * i + 1;
                    chain_length += 1;
                }
            }

            if chain_length > max_length {
                max_length = chain_length;
                start_number = curr;
            }
        }

        start_number
    }
}

// How many such routes are there through a 'n x n' grid?
pub mod p015 {
    use crate::useful_func::other_func::fact;

    pub fn v1(n: u32) -> u32 {
        // n choices to make among 2n -> C(2n,n) = (2n)! / n! (2n - n)!
        fact(2 * n) / (fact(n) * fact(n))
    }
}

// Find the sum of the digits of the number 2^n.
pub mod p016 {
    use crate::useful_func::digits::num_to_digits;

    pub fn v1(n: u32) -> u32 {
        num_to_digits(2_u32.pow(n)).iter().sum()
    }
}

// Find the number of letters used to write all the numbers from 1 to 'n' in words (in english).
pub mod p017 {
    pub fn v1(n: u32) -> u32 {
        let letters_0to19 = |num: u32| match num {
            1 | 2 | 6 | 10 => 3,
            0 | 4 | 5 | 9 => 4,
            3 | 7 | 8 => 5,
            11 | 12 => 6,
            15 | 16 => 7,
            13 | 14 | 18 | 19 => 8,
            17 => 9,
            _ => panic!(),
        };
        let letters_0to99 = |num: u32| match num {
            0..=19 => letters_0to19(num),
            40 | 50 | 60 => 5,
            41..=49 | 51..=59 | 61..=69 => 5 + letters_0to19(num % 20),
            20 | 30 | 80 | 90 => 6,
            21..=29 | 31..=39 | 81..=89 | 91..=99 => 6 + letters_0to19(num % 20),
            70 => 7,
            71..=79 => 7 + letters_0to19(num % 20),
            _ => panic!(),
        };
        let letters_0to999 = |num: u32| {
            let hundreds = num / 100;
            match num {
                0..=99 => letters_0to99(num),
                100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 => 7 + letters_0to19(hundreds),
                101..=999 => 7 + letters_0to19(hundreds) + 3 + letters_0to99(num % 100),
                _ => panic!(),
            }
        };
        let letters_0to1million = |num: u32| {
            let n_letters: u32;
            let thousands = num / 1000;
            if thousands > 0 && num % 1000 == 0 {
                n_letters = match num {
                    1_000_000 => 10,
                    _ => 8 + letters_0to999(thousands),
                }
            } else {
                n_letters = match num {
                    0..=999 => letters_0to999(num),
                    _ => 8 + letters_0to999(thousands) + letters_0to999(num % 1000),
                }
            }
            n_letters
        };
        let mut n_letters = 0;
        for i in 1..=n {
            n_letters += letters_0to1million(i);
        }
        n_letters
    }
}

// Given a triangle of number, find the maximum sum from top to bottom.
pub mod p018 {
    use crate::useful_func::digits::get_triangle_digits;
    use std::cmp::max;

    pub fn v1(str_triangle: &str) -> u32 {
        let mut num_triangle: Vec<Vec<u32>> = get_triangle_digits(str_triangle);
        let height = num_triangle.len();

        for x in 1..height {
            let width = x + 1;
            for y in 0..width {
                if y == 0 {
                    num_triangle[x][y] += num_triangle[x - 1][y];
                } else if y == x {
                    num_triangle[x][y] += num_triangle[x - 1][y - 1];
                } else {
                    num_triangle[x][y] += max(num_triangle[x - 1][y], num_triangle[x - 1][y - 1]);
                }
            }
        }

        *num_triangle.last().unwrap().iter().max().unwrap()
    }
}

// How many Sundays fell on the first of the month from 1 Jan 1901 to 31 Dec 2000?
pub mod p019 {
    struct Date {
        year: u32,
        month: u32,
        day: u32,
    }
    impl PartialEq for Date {
        fn eq(&self, other: &Self) -> bool {
            self.year == other.year && self.month == other.month && self.day == other.day
        }
    }
    pub fn v1() -> u32 {
        let days_since_1900 = |date: &Date| -> u32 {
            let mut n_days: u32;
            n_days = date.day - 1; // day passed this month
                                   // number of days in the months passed
            n_days += match date.month {
                1 => 0,
                2 => 31,
                3 => 31 + 28,
                4 => 2 * 31 + 28,
                5 => 2 * 31 + 28 + 30,
                6 => 3 * 31 + 28 + 30,
                7 => 3 * 31 + 28 + 2 * 30,
                8 => 4 * 31 + 28 + 2 * 30,
                9 => 5 * 31 + 28 + 2 * 30,
                10 => 5 * 31 + 28 + 3 * 30,
                11 => 6 * 31 + 28 + 3 * 30,
                12 => 6 * 31 + 28 + 4 * 30,
                _ => panic!(),
            };
            if date.year % 4 == 0 && date.month > 2 {
                n_days += 1;
            }
            // number of days in the years passed since 1900
            let n_years = date.year - 1900;
            n_days += n_years * 365;
            let n_leap_years = n_years / 4;
            n_days += n_leap_years - 1; // -1 because 1900 is no leap year
            n_days
        };
        let new_century: Date = Date {
            year: 2001,
            month: 1,
            day: 1,
        };
        let mut curr_date: Date = Date {
            year: 1901,
            month: 1,
            day: 1,
        };
        let mut counter: u32 = 0;
        while curr_date != new_century {
            let days_passed = days_since_1900(&curr_date);
            if days_passed % 7 == 0 {
                counter += 1;
            }
            // update curr_date
            if curr_date.month > 12 {
                curr_date.year += 1;
                curr_date.month = 1;
            } else {
                curr_date.month += 1;
            }
        }
        counter
    }
}

// Find the sum of the digits in the number n!
pub mod p020 {
    use crate::useful_func::digits::num_to_digits;

    pub fn v1(n: u32) -> u32 {
        num_to_digits((1..=n).product()).iter().sum()
    }
}
