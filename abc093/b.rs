use std::collections::HashSet;

fn main() {
    let v = read_vec::<i32>();
    let a = v[0];
    let b = v[1];
    let k = v[2];

    let mut hs = HashSet::new();
    for i in 0..k {
        if a + i > b {
            break;
        }
        hs.insert(a+i);
    }
    for j in 0..k {
        if b - j < a {
            break;
        }
        hs.insert(b-j);
    }
    let mut ans = hs.into_iter().collect::<Vec<_>>();
    ans.sort();
    for x in ans {
        println!("{}", x);
    }
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
