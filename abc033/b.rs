fn main() {
    let n = read::<usize>();
    let mut sum = 0;
    let mut names = vec![];
    for _ in 0..n {
        let v = read_vec::<String>();
        let s = v[0].clone();
        let p = v[1].parse::<u32>().unwrap();
        names.push((s, p));
        sum += p;
    }

    for (s, p) in names {
        if sum / p < 2 {
            println!("{}", s);
            return;
        }
    }
    println!("atcoder");
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
