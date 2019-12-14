fn main() {
    let n = read::<usize>();
    let mut v = vec![];
    for _ in 0..n {
        v.push(read::<usize>());
    }

    const IMPOSSIBLE: usize = 1000000;
    let mut min_count = IMPOSSIBLE;

    let mut count = 0;
    let mut j = 0;
    while j + 1 != v[j] && count <= n {
        count += 1;
        if v[j] == 2 {
            min_count = std::cmp::min(min_count, count);
        } else {
            j = v[j] - 1;
        }
    }

    if min_count == IMPOSSIBLE {
        println!("-1");
    } else {
        println!("{}", min_count);
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
