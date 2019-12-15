fn main() {
    let v = read_vec::<u32>();
    let n = v[0];
    let m = v[1];

    let gaku = read_vec2::<i32>(n);
    let check = read_vec2::<i32>(m);

    for g in gaku {
        let mut min = 1000000000;
        let mut minpoint = 0;
        for (i, c) in check.iter().enumerate() {
            let manhattan = (g[0] - c[0]).abs() + (g[1] - c[1]).abs();
            if min > manhattan {
                min = manhattan;
                minpoint = i + 1;
            }
        }
        println!("{}", minpoint);
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
