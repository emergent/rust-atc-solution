fn main() {
    let v = read_vec::<u64>();
    let n = v[0] as usize;
    let mut h = v[1];

    let mut aa = vec![];
    let mut bb = vec![];
    for _ in 0..n {
        let vv = read_vec::<u64>();
        aa.push(vv[0]);
        bb.push(vv[1]);
    }

    aa.sort();
    bb.sort();

    let a = aa[n-1];
    let mut ans = 0;

    while let Some(b) = bb.pop() {
        if a <= b {
            if h > b {
                h -= b;
                ans += 1;
            } else {
                h = 0;
                ans += 1;
                break;
            }
        } else {
            break;
        }
    }
    if h != 0 {
        ans += (h + a - 1) / a;
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
