// Problem 11: Largest product in a grid
//   Find the greatest product of four adjacent numbers in the same direction
//   (up, down, left, right, or diagonally) in the 20×20 grid.
// Problem 12: Highly divisible triangular number
//   Find the first triangle number to have over 'n' divisors
// Problem 13: Large sum
//   Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
// Problem 14: Longest Collatz sequence
//   Which starting number, under 'n', produces the longest chain?
// Problem 15: Lattice paths
//   How many such routes are there through a 'n x n' grid?
// Problem 16: Power digit sum
//   Find the sum of the digits of the number 2^n.
// Problem 17: Number letter counts
//   Find the number of letters used to write all the numbers from 1 to 'n' in words (in english).
// Problem 18: Maximum path sum I
//   Find the maximum total from top to bottom of the triangle below.
// Problem 19: Counting Sundays
//   How many Sundays fell on the first of the month from 1 Jan 1901 to 31 Dec 2000 ?
// Problem 20: Factorial digit sum
//   Find the sum of the digits in the number n!
pub mod p011 {
    pub fn v1() -> u32 {
        let grid: String = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08".to_string()
            + " 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00"
            + " 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65"
            + " 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91"
            + " 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80"
            + " 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50"
            + " 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70"
            + " 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21"
            + " 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72"
            + " 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95"
            + " 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92"
            + " 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57"
            + " 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58"
            + " 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40"
            + " 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66"
            + " 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69"
            + " 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36"
            + " 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16"
            + " 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54"
            + " 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";
        let mut grid_num: Vec<u32> = Vec::new();
        for (i, c) in grid.chars().enumerate() {
            if i % 3 == 0 {
                grid_num.push(c.to_digit(10).unwrap());
            } else if i % 3 == 1 {
                let last: u32 = grid_num.pop().unwrap();
                grid_num.push(last * 10 + c.to_digit(10).unwrap());
            }
        }
        let mut largest_prod = 0;
        let length = 20;
        let n_adj = 4;
        // Horizontal
        for row in 0..length {
            for col in 0..(length - n_adj) {
                let mut mul = 1;
                for i in 0..n_adj {
                    let index = row * length + (col + i);
                    mul *= grid_num[index];
                }
                if mul > largest_prod { largest_prod = mul; }
            }
        }
        // Vertical
        for col in 0..length {
            for row in 0..(length - n_adj) {
                let mut mul = 1;
                for i in 0..n_adj {
                    let index = (row + i) * length + col;
                    mul *= grid_num[index];
                }
                if mul > largest_prod { largest_prod = mul; }
            }
        }
        // Diagonal
        for col in 0..(length - n_adj) {
            for row in 0..(length - n_adj) {
                let mut mul = 1;
                for i in 0..n_adj {
                    let index = (row + i) * length + (col + i);
                    mul *= grid_num[index];
                }
                if mul > largest_prod { largest_prod = mul; }
            }
        }
        largest_prod
    }
}

pub mod p012 {
    pub fn v1(n: u32) -> u32 {
        let mut triangular = 1;
        let mut next_tri = 2;
        loop {
            let mut counter = 0;
            for i in 1..triangular+1 {
                if triangular % i == 0 { counter += 1; }
            }
            if counter >= n { break; }
            triangular += next_tri;
            next_tri += 1;
        }
        triangular
    }
}

pub mod p013 {
    pub fn v1() -> u32 {
        let base: u32 = 10;         // need 'base' because: 10.pow() gives an error and (10 as u32) is not supported or something -- it's probably optimized by the compiler anyway
        let number = "37107287533902102798797998220837590246510135740250".to_string()
            + "46376937677490009712648124896970078050417018260538"
            + "74324986199524741059474233309513058123726617309629"
            + "91942213363574161572522430563301811072406154908250"
            + "23067588207539346171171980310421047513778063246676"
            + "89261670696623633820136378418383684178734361726757"
            + "28112879812849979408065481931592621691275889832738"
            + "44274228917432520321923589422876796487670272189318"
            + "47451445736001306439091167216856844588711603153276"
            + "70386486105843025439939619828917593665686757934951"
            + "62176457141856560629502157223196586755079324193331"
            + "64906352462741904929101432445813822663347944758178"
            + "92575867718337217661963751590579239728245598838407"
            + "58203565325359399008402633568948830189458628227828"
            + "80181199384826282014278194139940567587151170094390"
            + "35398664372827112653829987240784473053190104293586"
            + "86515506006295864861532075273371959191420517255829"
            + "71693888707715466499115593487603532921714970056938"
            + "54370070576826684624621495650076471787294438377604"
            + "53282654108756828443191190634694037855217779295145"
            + "36123272525000296071075082563815656710885258350721"
            + "45876576172410976447339110607218265236877223636045"
            + "17423706905851860660448207621209813287860733969412"
            + "81142660418086830619328460811191061556940512689692"
            + "51934325451728388641918047049293215058642563049483"
            + "62467221648435076201727918039944693004732956340691"
            + "15732444386908125794514089057706229429197107928209"
            + "55037687525678773091862540744969844508330393682126"
            + "18336384825330154686196124348767681297534375946515"
            + "80386287592878490201521685554828717201219257766954"
            + "78182833757993103614740356856449095527097864797581"
            + "16726320100436897842553539920931837441497806860984"
            + "48403098129077791799088218795327364475675590848030"
            + "87086987551392711854517078544161852424320693150332"
            + "59959406895756536782107074926966537676326235447210"
            + "69793950679652694742597709739166693763042633987085"
            + "41052684708299085211399427365734116182760315001271"
            + "65378607361501080857009149939512557028198746004375"
            + "35829035317434717326932123578154982629742552737307"
            + "94953759765105305946966067683156574377167401875275"
            + "88902802571733229619176668713819931811048770190271"
            + "25267680276078003013678680992525463401061632866526"
            + "36270218540497705585629946580636237993140746255962"
            + "24074486908231174977792365466257246923322810917141"
            + "91430288197103288597806669760892938638285025333403"
            + "34413065578016127815921815005561868836468420090470"
            + "23053081172816430487623791969842487255036638784583"
            + "11487696932154902810424020138335124462181441773470"
            + "63783299490636259666498587618221225225512486764533"
            + "67720186971698544312419572409913959008952310058822"
            + "95548255300263520781532296796249481641953868218774"
            + "76085327132285723110424803456124867697064507995236"
            + "37774242535411291684276865538926205024910326572967"
            + "23701913275725675285653248258265463092207058596522"
            + "29798860272258331913126375147341994889534765745501"
            + "18495701454879288984856827726077713721403798879715"
            + "38298203783031473527721580348144513491373226651381"
            + "34829543829199918180278916522431027392251122869539"
            + "40957953066405232632538044100059654939159879593635"
            + "29746152185502371307642255121183693803580388584903"
            + "41698116222072977186158236678424689157993532961922"
            + "62467957194401269043877107275048102390895523597457"
            + "23189706772547915061505504953922979530901129967519"
            + "86188088225875314529584099251203829009407770775672"
            + "11306739708304724483816533873502340845647058077308"
            + "82959174767140363198008187129011875491310547126581"
            + "97623331044818386269515456334926366572897563400500"
            + "42846280183517070527831839425882145521227251250327"
            + "55121603546981200581762165212827652751691296897789"
            + "32238195734329339946437501907836945765883352399886"
            + "75506164965184775180738168837861091527357929701337"
            + "62177842752192623401942399639168044983993173312731"
            + "32924185707147349566916674687634660915035914677504"
            + "99518671430235219628894890102423325116913619626622"
            + "73267460800591547471830798392868535206946944540724"
            + "76841822524674417161514036427982273348055556214818"
            + "97142617910342598647204516893989422179826088076852"
            + "87783646182799346313767754307809363333018982642090"
            + "10848802521674670883215120185883543223812876952786"
            + "71329612474782464538636993009049310363619763878039"
            + "62184073572399794223406235393808339651327408011116"
            + "66627891981488087797941876876144230030984490851411"
            + "60661826293682836764744779239180335110989069790714"
            + "85786944089552990653640447425576083659976645795096"
            + "66024396409905389607120198219976047599490197230297"
            + "64913982680032973156037120041377903785566085089252"
            + "16730939319872750275468906903707539413042652315011"
            + "94809377245048795150954100921645863754710598436791"
            + "78639167021187492431995700641917969777599028300699"
            + "15368713711936614952811305876380278410754449733078"
            + "40789923115535562561142322423255033685442488917353"
            + "44889911501440648020369068063960672322193204149535"
            + "41503128880339536053299340368006977710650566631954"
            + "81234880673210146739058568557934581403627822703280"
            + "82616570773948327592232845941706525094512325230608"
            + "22918802058777319719839450180888072429661980811197"
            + "77158542502016545090413245809786882778948721859617"
            + "72107838435069186155435662884062257473692284509516"
            + "20849603980134001723930671666823555245252804609722"
            + "53503534226472524250874054075591789781264330331690";
        let n_digits = 50;
        let n_first = 10;
        let mut first_digits: u32 = 0;
        for (i, c) in number.chars().enumerate() {
            let position = i % n_digits;
            if position > n_digits - n_first {
                let exp = n_digits - position - 1;  // last digit: 50 - 49 - 1 = 0
                first_digits += c.to_digit(10).unwrap() * base.pow(exp as u32);
            }
        }
        first_digits % base.pow(9)
    }
}

pub mod p014 {
    pub fn v1(n: u32) -> u32 {
        let mut start_number = 1;
        let mut max_length = 1;
        for curr in 2..n+1 {
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

pub mod p015 {
    pub fn v1(n: u32) -> u32 {
        // n choices to make among 2n -> C(2n,n) = (2n)! / n! (2n - n)!
        let n_fact: u32 = (2..n+1).product();
        let two_n_fact: u32 = (2..(2*n+1)).product();
        two_n_fact / (n_fact * n_fact)
    }
}

pub mod p016 {
    pub fn v1(n: u32) -> u32 {
        let base: u32 = 2;                                          // need 'base' because: 2.pow() doesn't work
        let number = base.pow(n).to_string();       // base.pow(n).to_string().chars().to_digit(10).unwrap().sum()  ===>  to_digit() doesn't work on char iterator
        let mut sum = 0;
        for c in number.chars() {
            sum += c.to_digit(10).unwrap();
        }
        sum
    }
}

pub mod p017 {
    pub fn v1(n: u32) -> u32 {
        let letters_0to19 = |num: u32| {
            match num {
                1 | 2 | 6 | 10 => 3,
                0 | 4 | 5 | 9 => 4,
                3 | 7 | 8 => 5,
                11 | 12 => 6,
                15 | 16 => 7,
                13 | 14 | 18 | 19 => 8,
                17 => 9,
                _ => panic!(),
            }
        };
        let letters_0to99 = |num: u32| {
            match num {
                0...19 => letters_0to19(num),
                40 | 50 | 60 => 5,
                41...49 | 51...59 | 61...69 => 5 + letters_0to19(num % 20),
                20 | 30 | 80 | 90 => 6,
                21...29 | 31...39 | 81...89 | 91...99 => 6 + letters_0to19(num % 20),
                70 => 7,
                71...79 => 7 + letters_0to19(num % 20),
                _ => panic!(),
            }
        };
        let letters_0to999 = |num: u32| {
            let hundreds = num / 100;
            match num {
                0...99 => letters_0to99(num),
                100 | 200 | 300 | 400 | 500 | 600 | 700
                    | 800 | 900 => 7 + letters_0to19(hundreds),
                101...999 => 7 + letters_0to19(hundreds) + 3 + letters_0to99(num % 100),
                _ => panic!(),
            }
        };
        let letters_0to1million = |num: u32| {
            let n_letters: u32;
            let thousands = num / 1000;
            if thousands > 0 && num % 1000 == 0 {
                n_letters = match num {
                    1000_000 => 10,
                    _ => 8 + letters_0to999(thousands),
                }
            } else {
                n_letters = match num {
                    0...999 => letters_0to999(num),
                    _ => 8 + letters_0to999(thousands) + letters_0to999(num % 1000),
                }
            }
            n_letters
        };
        let mut n_letters = 0;
        for i in 1..n+1 {
            n_letters += letters_0to1million(i);
        }
        n_letters
    }
}

pub mod p018 {
    pub fn v1() -> u32 {
        let triangle_string: String = "75".to_string()
            + " 95 64"
            + " 17 47 82"
            + " 18 35 87 10"
            + " 20 04 82 47 65"
            + " 19 01 23 75 03 34"
            + " 88 02 77 73 07 63 67"
            + " 99 65 04 28 06 16 70 92"
            + " 41 41 26 56 83 40 80 70 33"
            + " 41 48 72 33 47 32 37 16 94 29"
            + " 53 71 44 65 25 43 91 52 97 51 14"
            + " 70 11 33 28 77 73 17 78 39 68 17 57"
            + " 91 71 52 38 17 14 91 43 58 50 27 29 48"
            + " 63 66 04 68 89 53 67 30 73 16 69 87 40 31"
            + " 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
        let mut triangle: Vec<Vec<u32>> = Vec::new();
        for (i, c) in triangle_string.chars().enumerate() {
            if i % 3 == 0 {
                // unwrap safely by checking the length of triangle beforehand
                if triangle.len() == 0 || triangle.last().unwrap().len() >= triangle.len(){
                    triangle.push(vec!(c.to_digit(10).unwrap()));
                } else {
                    let mut line = triangle.pop().unwrap();
                    line.push(c.to_digit(10).unwrap());
                }
            } else if i % 3 == 1 {
                let size = triangle.len();
                let last_num = triangle[size-1].pop().unwrap();
                triangle[size-1].push(last_num * 10 + c.to_digit(10).unwrap());
            }
        }
        for i in 1..triangle.len() {
            for j in 0..i {
                if j == 0 {
                    triangle[i][j] += triangle[i-1][j];
                } else if j == i {
                    triangle[i][j] += triangle[i-1][j-1];
                } else {
                    let max: u32;
                    if triangle[i-1][j] > triangle[i-1][j-1] {
                        max = triangle[i-1][j];
                    } else {
                        max = triangle[i-1][j-1]
                    }
                    triangle[i][j] += max;
                }
            }
        }
        *triangle.last().unwrap().iter().max().unwrap()
    }
}

pub mod p019 {
    struct Date {
        year: u32,
        month: u32,
        day: u32,
    }
    impl PartialEq for Date {
        fn eq(&self, other: &Self) -> bool {
            self.year == other.year && self.month == other.month
                && self.day == other.day
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
                4 => 2*31 + 28,
                5 => 2*31 + 28 + 30,
                6 => 3*31 + 28 + 30,
                7 => 3*31 + 28 + 2*30,
                8 => 4*31 + 28 + 2*30,
                9 => 5*31 + 28 + 2*30,
                10 => 5*31 + 28 + 3*30,
                11 => 6*31 + 28 + 3*30,
                12 => 6*31 + 28 + 4*30,
                _ => panic!(),
            };
            if date.year % 4 == 0 && date.month > 2 { n_days += 1; }
            // number of days in the years passed since 1900
            let n_years = date.year - 1900;
            n_days += n_years * 365;
            let n_leap_years = n_years / 4;
            n_days += n_leap_years - 1; // -1 because 1900 is no leap year
            n_days
        };
        let new_century: Date = Date { year: 2001, month: 1, day: 1 };
        let mut curr_date: Date = Date { year: 1901, month: 1, day: 1 };
        let mut counter: u32 = 0;
        while curr_date != new_century {
            let days_passed = days_since_1900(&curr_date);
            if days_passed % 7 == 0 { counter += 1; }
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

pub mod p020 {
    pub fn v1(n: u32) -> u32 {
        let fact: u32 = (2..n+1).product();
        let mut sum: u32 = 0;
        for c in fact.to_string().chars() {
            sum += c.to_digit(10).unwrap();
        }
        sum
        // sum = fact.to_string().chars().to_digit(10).unwrap().sum();
    }
}
