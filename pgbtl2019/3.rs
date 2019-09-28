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
    let v = read_vec::<u32>();
    let n = v[0];
    let m = v[1];
    let abs = read_vec2::<u32>(n);
    let mut pins = vec![0u32; m as usize];
    for (t, ab) in abs.iter().enumerate() {
        let p = t as i32 - (ab[0] + ab[1]) as i32;
        if p >= 0 && p < m as i32 {
            pins[p as usize] = 1;
            //println!("{}: {:?}", t, pins);
        }
    }

    let ans = pins.into_iter().sum::<u32>();
    println!("{}", ans);
}
