fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let m = v[1];
    let mut vv = vec![0usize; n];

    for i in 0..m {
        let vvv = read_vec::<usize>();
        vv[vvv[0] - 1] += 1;
        vv[vvv[1] - 1] += 1;
    }
    for a in vv {
        println!("{}", a);
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
