fn main() {
    let n = read::<usize>();
    let mut v = vec![];
    let mut sum = 0;
    for i in 0..n {
        let s = read::<u32>();
        sum += s;
        v.push(s);
    }
    if sum % 10 != 0 {
        println!("{}", sum);
        return;
    } else {
        v.sort();
        for m in v {
            if (sum - m) % 10 != 0 {
                println!("{}", sum - m);
                return;
            }
        }
    }
    println!("{}", 0);
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
