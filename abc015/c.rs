fn check(n: usize, ts: &Vec<Vec<u32>>, ansv: &mut Vec<u32>) {
    let tsize = ts.len();
    let aa = ansv.clone();
    if n == 0 {
        return;
    }

    if tsize - n == 0 {
        for &t in &ts[0] {
            ansv.push(t);
        }
    } else {
        ansv.clear();
        for &a in &aa {
            for &t in &ts[tsize - n] {
                ansv.push(t ^ a);
            }
        }
    }

    check(n - 1, ts, ansv);
}

fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let _k = v[1];
    let mut ts = vec![];
    for _ in 0..n {
        ts.push(read_vec::<u32>());
    }
    let mut ansv = vec![];
    check(n, &ts, &mut ansv);

    if ansv.contains(&0) {
        println!("Found");
    } else {
        println!("Nothing");
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
