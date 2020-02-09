fn main() {
    let v = read_vec::<u32>();
    let n = v[0];
    let _k = v[1];
    let ds = read_vec::<u32>();
    let mut ans = 0;
    for i in n.. {
        let mut is_ok = true;
        let mut j = i;
        while j > 0 {
            if ds.contains(&(j % 10)) {
                is_ok = false;
                break;
            }
            j /= 10;
        }
        if is_ok {
            ans = i;
            break;
        }
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
