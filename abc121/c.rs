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
    let v = read_vec::<i32>();
    let n = v[0] as usize;
    let m = v[1] as usize;
    let mut ab = read_vec2::<u64>(n as u32);

    ab.sort_by_key(|vv| vv[0]);
    let mut mrest = m as u64;
    let mut ans = 0;
    for i in 0.. {
        if mrest <= 0 {
            break;
        }

        let a = ab[i][0];
        let b = ab[i][1];

        if b >= mrest {
            ans += a * mrest;
            mrest = 0;
        } else {
            ans += a * b;
            mrest -= b;
        }
    }
    println!("{}", ans);
}
