fn main() {
    let n = read::<u32>();
    let m = (n / 5) % 6;
    let k = n % 5;
    let mut v = vec![1, 2, 3, 4, 5, 6];
    for _ in 0..m {
        let tmp = v.remove(0);
        v.push(tmp);
    }
    for j in 0..k as usize {
        let tmp = v[j % 5];
        v[j % 5] = v[j % 5 + 1];
        v[j % 5 + 1] = tmp;
    }
    let ans = v
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join("");
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
