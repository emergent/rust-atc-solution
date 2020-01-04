use std::collections::*;
fn is753(n: u32) -> bool {
    let mut c3 = 0;
    let mut c5 = 0;
    let mut c7 = 0;
    let mut m = n;
    while m > 0 {
        match m % 10 {
            3 => c3 += 1,
            5 => c5 += 1,
            7 => c7 += 1,
            _ => return false,
        }
        m /= 10;
    }
    c3 > 0 && c5 > 0 && c7 > 0
}
fn add753(v: &mut Vec<u32>) {
    let vlen = v.len();
    let w = v.clone();
    let v753 = vec![7, 5, 3];
    for x in v753 {
        for i in 0..vlen {
            v.push(w[i] * 10 + x);
        }
    }
}

fn main() {
    let n = read::<u32>();

    if n < 357 {
        println!("0");
        return;
    }

    let mut keta = 0;
    let mut m = n;
    while m > 0 {
        keta += 1;
        m /= 10;
    }

    let v753: Vec<u32> = vec![3, 5, 7];
    let mut v = vec![];
    for x in &v753 {
        for y in &v753 {
            for z in &v753 {
                v.push(x * 100 + y * 10 + z);
            }
        }
    }
    for _ in 4..keta + 1 {
        add753(&mut v);
    }
    let ansv = v
        .into_iter()
        .filter(|&x| is753(x) && x <= n)
        .collect::<HashSet<u32>>();
    //println!("{:?}", ansv);
    println!("{}", ansv.len());
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
