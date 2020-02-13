use std::collections::HashMap;

fn factors(hm: &mut HashMap<u64, u64>, mut x: u64) {
    let n = x;
    for i in 2..n + 1 {
        let c = hm.entry(i).or_insert(1);
        while x % i == 0 {
            *c += 1;
            x /= i;
        }
        if n == 1 {
            break;
        }
    }
}

fn main() {
    let n = read::<u64>();
    let mut hm = HashMap::new();
    for i in 1..n + 1 {
        factors(&mut hm, i);
    }

    //println!("{:?}", hm);

    let mut count = 1;
    for &v in hm.values() {
        count *= v;
        count %= 1000000007;
    }

    println!("{}", count);
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
#[allow(dead_code)]
fn yn(result: bool) {
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
