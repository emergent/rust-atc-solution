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
    let v = read_vec::<i32>();
    let n = v[0] as usize;
    let m = v[1] as usize;
    let c = v[2];

    let b = read_vec::<i32>();
    let a = read_vec2::<i32>(n as u32);

    let mut ans = 0;
    for i in 0..n {
        let mut ans_i = 0;
        for j in 0..m {
            ans_i += a[i][j] * b[j]
        }
        if ans_i + c > 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
