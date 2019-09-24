// 借鉴第24题和37题
fn main() {
    let mut v = [1, 2, 3, 4, 5, 6, 7];
    let mut max_prime = 0;
    // 不是特别严谨，第1个组合被忽略了
    while next_perm(&mut v) {
        //let v_str = v.iter().map(|x| x.to_string()).collect::<String>();
        //let d = v_str.parse::<usize>().unwrap();
        let d = v.iter().fold(0, |x, a| 10 * x + a);
        if primes::is_prime(d as u64) && d > max_prime {
            println!("{}", d);
            max_prime = d;
        }
    }
}
// 7652413

fn next_perm(v: &mut [u64]) -> bool {
    let mut i = v.len() - 2;
    while v[i] > v[i + 1] {
        if i == 0 {
            return false;
        }
        i -= 1;
    }

    let mut j = v.len() - 1;
    while i < j && v[i] > v[j] {
        j -= 1;
    }

    swap(v, i, j);

    i += 1;
    j = v.len() - 1;
    while i < j {
        swap(v, i, j);
        i += 1;
        j -= 1;
    }
    true
}

fn swap(v: &mut [u64], i: usize, j: usize) {
    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}
