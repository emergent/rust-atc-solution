fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

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

fn main() {
    let n = read::<usize>();
    let mut ab = Vec::new();
    for _ in 0..n {
        let v = read_vec::<u32>();
        let a = v[0];
        let b = v[1];
        ab.push((a, b));
    }

    ab.sort_by_key(|&(_, b)| b);

    let mut ans_f = true;
    let mut time = 0;
    for (a, b) in ab {
        time += a;
        if b < time {
            ans_f = false;
        }
    }

    if ans_f {
        println!("Yes");
    } else {
        println!("No");
    }
}
