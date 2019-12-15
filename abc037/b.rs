fn main() {
    let v = read_vec::<u32>();
    let n = v[0];
    let q = v[1];
    let mut ansv = vec![0u32; n as usize];

    for _ in 0..q {
        let lrt = read_vec::<u32>();
        let l = lrt[0] as usize;
        let r = lrt[1] as usize;
        let t = lrt[2];

        for j in l - 1..r {
            ansv[j] = t;
        }
    }
    for a in ansv {
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
