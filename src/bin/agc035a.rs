fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

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

use std::collections::HashSet;
fn main() {
    let n = read::<usize>();
    let a = read_vec::<i64>();
    if a.iter().all(|&i| i == 0) {
        yn(true);
        return;
    }

    let hs = a.iter().map(|i| *i).collect::<HashSet<i64>>();
    let count = hs.len();
    if !(count == 2 || count == 3) {
        yn(false);
        return;
    }
    if count == 3 {
        let v = hs.iter().map(|i| *i).collect::<Vec<i64>>();
        if v[0] ^ v[1] != v[2] || v[1] ^ v[2] != v[0] || v[2] ^ v[0] != v[1] {
            yn(false);
            return;
        }
        let mut vc = [0u32; 3];
        for i in 0..n {
            if a[i] == v[0] {
                vc[0] += 1;
            } else if a[i] == v[1] {
                vc[1] += 1;
            } else {
                vc[2] += 1;
            }
        }
        if vc[0] == vc[1] && vc[1] == vc[2] {
            yn(true);
            return;
        } else {
            yn(false);
            return;
        }
    } else {
        let mut v = hs.iter().map(|i| *i).collect::<Vec<i64>>();
        v.sort();
        if v[0] != 0 {
            yn(false);
            return;
        }
        let mut vc = [0u32; 2];
        for i in 0..n {
            if a[i] == v[0] {
                vc[0] += 1;
            } else {
                vc[1] += 1;
            }
        }
        if vc[0] * 2 == vc[1] {
            yn(true);
            return;
        } else {
            yn(false);
            return;
        }
    }
}
