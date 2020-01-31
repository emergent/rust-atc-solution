use std::collections::HashMap;
fn main() {
    let n = read::<usize>();
    let mut v = vec![];
    let mut hmall = HashMap::new();
    let HASH_MAX = 10000;
    for _ in 0..n {
        let s = read::<String>().chars().collect::<Vec<_>>();
        let mut hm = HashMap::new();
        for c in s {
            let cc = hm.entry(c).or_insert(0);
            *cc += 1;

            let cc = hmall.entry(c).or_insert(HASH_MAX);
        }
        v.push(hm);
    }
    //println!("{:?}", v);

    let hmall2 = hmall.clone();
    //println!("{:?}", hmall2);
    for (key, val) in hmall2 {
        for i in 0..n {
            if let Some(cv) = v[i].get(&key) {
                let c = hmall.entry(key).or_insert(0);
                if *c > *cv {
                    *c = *cv;
                }
            } else {
                hmall.insert(key, 0);
            }
        }
    }
    //println!("{:?}", hmall);

    let mut ansv = vec![];
    for &k in hmall.keys() {
        if let Some(&cc) = hmall.get(&k) {
            if cc != HASH_MAX {
                for _ in 0..cc {
                    ansv.push(k);
                }
            }
        }
    }
    ansv.sort();
    let ans = ansv.into_iter().collect::<String>();
    println!("{}", ans);
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
