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

fn gcd(x: u64, y: u64) -> u64 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

pub fn primes(x: u64) -> Vec<u64> {
    let x_ = x as usize;
    let mut ps = vec![];
    let mut is_prime = vec![true; x_ + 1];

    if x >= 2 {
        for i in 2..x_ + 1 {
            if is_prime[i] {
                ps.push(i as u64);
            }
            if i * i <= x_ {
                for j in 2..(x_ / i) + 1 {
                    is_prime[i * j] = false;
                }
            }
        }
    }
    ps
}

use std::collections::HashSet;
fn factors(n: u64, ps: &Vec<u64>) -> u64 {
    let mut m = n;
    let mut hs = HashSet::new();
    for &p in ps {
        while m % p == 0 {
            hs.insert(p);
            m /= p;
        }

        if m == 1 {
            break;
        }
    }
    if m != 1 {
        hs.insert(m);
    }
    //println!("{}, {:?}, {:?}", n, ps, hs);
    hs.len() as u64
}

fn main() {
    let v = read_vec::<u64>();
    let a = v[0];
    let b = v[1];

    let g = gcd(a, b);
    let root_g = {
        let mut i: u64 = 1;
        while i*i < g {
            i+=1;
        }
        i
    };

    let ps = primes(root_g + 2);
    //println!("{:?}", ps);
    let ans = factors(g, &ps);
    println!("{}", ans + 1);
}
