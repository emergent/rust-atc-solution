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
    let k = v[1];

    let mut hs: Vec<u32> = vec![];
    for _ in 0..n {
        hs.push(read::<u32>());
    }
    hs.sort();

    let mut v1 = vec![];
    for i in 0..(n - k + 1) {
        v1.push(hs[i + k - 1] - hs[i]);
    }
    let ans = v1.iter().min().unwrap();
    println!("{}", ans);
}
