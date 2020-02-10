fn main() {
    let s = read::<String>();
    let len = s.len();

    let mut ans = 0;
    for i in 0..1 << len - 1 {
        let mut start = 0;
        let mut sum = 0;

        for j in 0..len - 1 {
            if (1 << j) & i != 0 {
                sum += s[start..j + 1].parse::<u64>().unwrap();
                start = j + 1;
            }
        }
        sum += s[start..len].parse::<u64>().unwrap();
        ans += sum;
    }
    println!("{}", ans);
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
