fn main() {
    let n = read::<usize>();
    let mut nxn = vec![];
    for _ in 0..n {
        nxn.push(read::<String>().chars().collect::<Vec<_>>());
    }
    let mut ans = vec![vec!['x'; n]; n];
    for i in 0..n {
        for j in 0..n {
            ans[j][n - i - 1] = nxn[i][j];
        }
    }
    for a in ans {
        println!("{}", a.into_iter().collect::<String>());
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
