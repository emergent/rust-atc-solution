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
    let n = read::<usize>();
    let mut v = read_vec::<f64>();
    loop {
        v.sort_by(|v1, v2| v1.partial_cmp(v2).unwrap());
        let a = v.remove(0);
        let b = v.remove(0);
        v.push((a + b) / 2.0);

        if v.len() == 1 {
            break;
        }
    }
    println!("{}", v[0]);
}
