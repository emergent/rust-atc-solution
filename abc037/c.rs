fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let k = v[1];
    let aa = read_vec::<u64>();

    let m = std::cmp::min(n - k + 1, k);
    let mut bb = vec![m as u64; n];
    //println!("{:?}", bb);
    for i in 0..m {
        bb[i] = (i + 1) as u64;
        bb[n - i - 1] = (i + 1) as u64;
    }
    //println!("{:?}", bb);
    let mut sum: u64 = 0;
    for i in 0..n {
        sum += aa[i] * bb[i];
    }
    println!("{}", sum);
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
