fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn alleven(v: &Vec<i32>) -> bool {
    for i in v.iter() {
        if i % 2 != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let _n = read::<i32>();
    let mut v = read_vec::<i32>();

    let mut c = 0;
    while alleven(&v) {
        v = v.into_iter().map(|i| i / 2).collect();
        c += 1;
    }
    println!("{}", c);
}
