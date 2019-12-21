fn main() {
    let a = read::<u32>();
    let b = read::<u32>();
    let c = read::<u32>();
    let mut v = vec![a, b, c];
    let mut rank = vec![0, 0, 0];
    v.sort();
    for i in 0..3 {
        if v[i] == a {
            rank[0] = 3 - i;
        }
        if v[i] == b {
            rank[1] = 3 - i;
        }
        if v[i] == c {
            rank[2] = 3 - i;
        }
    }
    for r in rank {
        println!("{}", r);
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
