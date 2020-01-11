fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let m = v[1];

    let mut ks = vec![];
    for _ in 0..m {
        let mut vv = read_vec::<u32>();
        vv.remove(0);
        ks.push(vv);
    }

    let ps = read_vec::<u32>();

    let mut ans = 0;
    for i in 0..1 << n {
        let mut f = true;
        for j in 0..m {
            let mut count = 0;
            for k in &ks[j] {
                if (1 << (k - 1)) & i != 0 {
                    count += 1;
                }
            }
            if count % 2 != ps[j] {
                f = false;
                break;
            }
        }
        if f {
            ans += 1;
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
