/*
折腾的过程：
1）直接按照题意，可以非常容易写出来，10000以内很快
但数值大了之后，就会溢出
2）改算法，提前生成素数表，但当数字大的时候，生成素数都非常花时间，还是算不出来，重复计算的地方太多
3）梳理思路，把前面的一些数打印出来，分析规律，发现满足要求的数并不多
前面的字头确定了之后，后面不会跑出那些范围
4）可以用递归的方法，也不用提前生成素数，临时判断就行
release模式，几秒算完，算法的力量
*/
fn main() {
    let max_number_to_check = 1_000_000;
    let mut prime_mask = vec![true; max_number_to_check];
    prime_mask[0] = false;
    prime_mask[1] = false;
    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..max_number_to_check {
        if prime_mask[p] {
            let mut i = 2 * p;
            while i < max_number_to_check {
                prime_mask[i] = false;
                i += p;
            }
        }
    }
    println!("finished prime generation");

//    for n in 10..100000 {
//         if right_harshad(n) {
//                     println!("{}", n);
//         }
//     }
for i in 1..=9 {
right_har(i);
}

/*
    let mut sum = 0;
    for n in 10..max_number_to_check/10 {
    //println!("{}", harshad(201));
    //println!("{}", right_harshad(201));
    //println!("{}", strong_right_harshad(&prime_mask, 201));
        if strong_right_harshad(&prime_mask, n) {
            println!("strong: {}", n);
            for i in (1..=9).step_by(2) {
                let temp = (n as usize) * 10 + i;
                if prime_mask[temp] {
                    println!("{}", temp);
                    sum += temp;
                }
            }
        }
    }
    println!("sum: {}", sum);
    */
}

fn right_har(start: u64) {
    if start >= 9_999_999_999_999 {return;}
    if right_harshad(start) {
        if strong(start as usize) {
            //println!("{}", start);
            for j in (1..=9).step_by(2) {
                let temp = start * 10 + j;
                if primes::is_prime(temp) {
                    println!("{}", temp);
                    //sum += temp;
                }
            }
        }
        for i in 0..=9 {
            right_har(start * 10 + i);
        }
    }
}
fn harshad(n: u64) -> bool {
    let sum_digits: u32 = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
    n % (sum_digits as u64) == 0
}

/*
fn is_prime(n:u64) -> bool {
    for i in 2..(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
*/

fn right_harshad(n: u64) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut num:u64 = 0;
    let mut sum:u64 = 0;
    for d in digits {
        num = num * 10 + d as u64;
        sum += d as u64;
        if num % sum != 0 {
            return false;
        }
    }
    true
}

fn strong_right_harshad(prime_mask: &Vec<bool>, n: usize) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut num = 0;
    let mut sum = 0;
    for d in digits {
        num = num * 10 + d;
        sum += d;
        if num % sum != 0 {
            return false;
        }
    }
    prime_mask[(n as usize) / (sum as usize)]
}

fn strong(n: usize) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut num :u64= 0;
    let mut sum :u64= 0;
    for d in digits {
        num = num * 10 + d as u64;
        sum += d as u64;
        if num % sum != 0 {
            return false;
        }
    }
    primes::is_prime((n as u64) / (sum as u64))
}