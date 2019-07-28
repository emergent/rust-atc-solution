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

fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let m = v[1];

    let mut vs = vec![0usize; m];
    for _i in 0..n {
        let v2 = read_vec::<usize>();
        for j in 1..(v2[0] + 1) {
            vs[v2[j] - 1] += 1;
        }
    }

    let ans = vs.into_iter().filter(|&i| i == n).count();
    println!("{}", ans);
}
